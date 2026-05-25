use core::panic;
use std::{
    fs::read_to_string,
    sync::{Arc, RwLock, atomic::AtomicBool},
};

use futures_util::StreamExt;

use reqwest::Client;

use serde::{Deserialize, Serialize};

/// 消息角色枚举，定义对话中消息的角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// 推理努力程度枚举，控制模型的推理深度
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
    Max,
}

/// 思考类型枚举，控制是否启用思考模式
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    Enabled,
    Disabled,
}

/// 工具调用结构体，描述一次工具调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolFunctionCall,
}

/// 工具函数调用结构体，包含函数名和参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunctionCall {
    pub name: String,
    pub arguments: String,
}

/// 显示项枚举，用于前端展示不同类型的内容
pub enum DisplayItem {
    System(String),
    User(String),
    Think(String),
    Chat(String),
    ToolCall(String),
    ToolResult(String),
}

/// 消息结构体，代表对话中的一条消息
#[derive(Serialize, Clone)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing, default)]
    pub think_content: Option<String>,
}

/// 思考配置结构体，配置思考模式
#[derive(Serialize, Clone)]
pub struct ThinkingConfig {
    #[serde(rename = "type")]
    pub kind: ThinkingType,
}

/// 工具结构体，描述一个可用工具
#[derive(Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: FunctionDef,
}

/// 函数定义结构体，描述工具函数的信息
#[derive(Serialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// 请求体结构体，用于发送给LLM API的请求
#[derive(Serialize)]
struct RequestBody {
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    stream: bool,
}

/// LLM结构体，负责与语言模型进行交互
#[derive(Clone)]
pub struct LLM {
    client: Client,
    api_key: String,
    pub history: Arc<RwLock<Vec<Message>>>,
    max_history: usize,
    endpoint: String,
    pub abort: Arc<AtomicBool>,
}

impl Message {
    /// 将消息转换为显示项列表，用于前端展示
    pub fn to_display_items(&self) -> Vec<DisplayItem> {
        let mut items = Vec::new();
        match self.role {
            Role::System => {
                if let Some(c) = &self.content {
                    items.push(DisplayItem::System(format!("[System] {}", c)));
                }
            }

            Role::User => {
                if let Some(c) = &self.content {
                    items.push(DisplayItem::User(c.clone()));
                }
            }

            Role::Assistant => {
                if let Some(t) = &self.think_content {
                    items.push(DisplayItem::Think(t.clone()));
                }
                if let Some(tc) = &self.tool_calls {
                    for call in tc {
                        items.push(DisplayItem::ToolCall(format!("{:?}", call)));
                    }
                }
                if let Some(c) = &self.content {
                    items.push(DisplayItem::Chat(c.clone()));
                }
            }

            Role::Tool => {
                if let Some(c) = &self.content {
                    items.push(DisplayItem::Chat(c.clone()));
                }
            }
        }
        items
    }
}

impl LLM {
    /// 创建一个新的LLM实例，初始化API客户端、历史记录等
    pub fn new(api_key: String, endpoint: String) -> Self {
        let prompt = read_to_string("./debug/Prompt.txt").unwrap();
        Self {
            client: Client::new(),
            api_key,
            history: Arc::new(RwLock::new(vec![Message {
                role: Role::System,
                content: Some(prompt),

                tool_calls: None,
                tool_call_id: None,
                think_content: None,
            }])),
            max_history: 12,
            endpoint,
            abort: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 清空历史记录，只保留系统提示
    pub fn clear(&mut self) {
        self.history.write().unwrap().truncate(1);
    }

    /// 裁剪历史记录，保持在最大长度内
    pub fn trim(&mut self) {
        while self.history.read().unwrap().len() > self.max_history {
            self.history.write().unwrap().remove(1);
        }
    }

    /// 打断输出
    pub fn abort(&mut self) {
        self.abort.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// 撤回上一次对话
    pub fn withdraw(&mut self) -> String {
        match self.history.write() {
            Ok(mut w) => {
                while let Some(msg) = w.pop() {
                    match msg.role {
                        Role::User => return msg.content.unwrap(),
                        Role::System => {
                            w.push(msg);
                            return String::new();
                        }
                        _ => {}
                    }
                }
                String::new()
            }
            Err(e) => {
                eprintln!("{}", e);
                panic!()
            }
        }
    }

    /// 发送用户输入并与LLM进行对话，返回流式响应
    pub async fn chat(&mut self, input: String) -> anyhow::Result<()> {
        {
            let mut h = self.history.write().unwrap();
            h.push(Message {
                role: Role::User,
                content: Some(input),

                tool_call_id: None,
                tool_calls: None,
                think_content: None,
            });
        }

        let body = RequestBody {
            model: "deepseek-v4-flash".to_string(),
            thinking: None,
            reasoning_effort: None,
            messages: self
                .history
                .read()
                .unwrap()
                .iter()
                .map(|msg| Message {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                    tool_calls: msg.tool_calls.clone(),
                    tool_call_id: msg.tool_call_id.clone(),
                    think_content: None,
                })
                .collect(),
            tools: None,
            tool_choice: None,
            stream: true,
        };

        let result = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let mut stream = result.bytes_stream();

        {
            let mut h = self.history.write().unwrap();
            h.push(Message {
                role: Role::Assistant,
                content: Some(String::new()),
                tool_calls: None,
                tool_call_id: None,
                think_content: Some(String::new()),
            });
        }
        while let Some(chunk) = stream.next().await {
            if self.abort.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            let chunk = chunk.unwrap();

            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        return Ok(());
                    }

                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            match self.history.write() {
                                Ok(mut h) => {
                                    let len = h.len() - 1;
                                    if let Some(ref mut s) = h[len].content {
                                        s.push_str(content);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    panic!()
                                }
                            }
                            std::io::Write::flush(&mut std::io::stdout())?;
                        }
                        if let Some(content) =
                            json["choices"][0]["delta"]["reasoning_content"].as_str()
                        {
                            match self.history.write() {
                                Ok(mut h) => {
                                    let len = h.len() - 1;
                                    if let Some(ref mut s) = h[len].think_content {
                                        s.push_str(content);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    panic!()
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
