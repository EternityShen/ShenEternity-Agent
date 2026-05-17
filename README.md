# ShenEternity-Agent

一个基于 Rust 构建的终端 LLM 聊天客户端，使用 `ratatui` 和 `crossterm` 打造现代化的 TUI 界面。

## ✨ 特性

- 🖥️ **终端 UI 界面** - 基于 ratatui 构建的美观终端界面
- 💬 **流式聊天** - 支持流式接收 LLM 响应，实时显示
- 🔄 **多模式切换** - Normal/Input/CurrChat/Scroll 四种交互模式
- 📜 **聊天历史** - 自动管理对话历史，支持浏览和滚动
- ⌨️ **键盘驱动** - 完全基于键盘操作，高效便捷
- 🎨 **彩色界面** - 丰富的颜色主题和边框样式

## 📦 安装

### 前置要求

- Rust 1.70+ (Edition 2024)
- Cargo 包管理器

### 构建

```bash
# 克隆项目
git clone <repository-url>
cd sheneternity-agent

# 构建项目
cargo build --release

# 运行
cargo run --release
```

## 🚀 使用

### 环境变量

运行前需要设置 API Key：

```bash
export API_KEY="your-api-key-here"
```

### 启动

```bash
cargo run
```

应用启动后会连接到默认的本地 LLM 服务端点：`http://127.0.0.1:8080/v1/chat/completions`

## ⌨️ 快捷键

### 模式切换

使用 `Tab` 键在不同模式间循环切换：

```
Normal → Input → CurrChat → ScrollChat → Normal
```

### 各模式操作

| 模式 | 快捷键 | 功能 |
|------|--------|------|
| **Normal** | `Tab` | 切换到 Input 模式 |
| | `Esc` | 退出应用 |
| **Input** | 任意字符 | 输入消息内容 |
| | `Backspace` | 删除字符 |
| | `Enter` | 发送消息 |
| | `Tab` | 切换到 CurrChat 模式 |
| **CurrChat** | `j` | 下一条消息 |
| | `k` | 上一条消息 |
| | `Tab` | 切换到 ScrollChat 模式 |
| **ScrollChat** | `j` | 向下滚动 |
| | `k` | 向上滚动 |
| | `Tab` | 切换到 Normal 模式 |

## 🏗️ 项目结构

```
sheneternity-agent/
├── src/
│   ├── main.rs              # 应用入口，终端初始化
│   ├── lib.rs               # 模块导出
│   ├── data/
│   │   ├── mod.rs           # 数据模块
│   │   ├── app.rs           # 应用状态管理
│   │   └── llm.rs           # LLM API 客户端
│   ├── event/
│   │   ├── mod.rs           # 事件模块
│   │   └── keyboard.rs      # 键盘事件处理
│   ├── ui/
│   │   ├── mod.rs           # UI 模块
│   │   ├── layout.rs        # 主布局渲染
│   │   └── component/
│   │       ├── mod.rs       # 组件模块
│   │       ├── app_mode.rs  # 模式显示组件
│   │       ├── chat_window.rs    # 聊天窗口组件
│   │       ├── input_bar.rs      # 输入框组件
│   │       ├── current_chat_list.rs  # 当前聊天列表
│   │       └── histoy_caht_list.rs   # 历史聊天列表
│   └── utils/
│       └── mod.rs           # 工具函数
├── Cargo.toml               # 项目配置
└── Cargo.lock               # 依赖锁定文件
```

## 🔧 技术栈

- **[ratatui](https://github.com/ratatui-org/ratatui)** - Rust 终端 UI 框架
- **[crossterm](https://github.com/crossterm-rs/crossterm)** - 跨平台终端操作库
- **[tokio](https://github.com/tokio-rs/tokio)** - 异步运行时
- **[reqwest](https://github.com/seanmonstar/reqwest)** - HTTP 客户端
- **[serde](https://github.com/serde-rs/serde)** - 序列化/反序列化

## 📝 配置

### 修改 API 端点

编辑 [src/data/app.rs](file:///home/sheneternity/Work/Project/sheneternity-agent/src/data/app.rs#L30-L32) 中的默认端点：

```rust
llm_handle: LLM::new(
    std::env::var("API_KEY").unwrap_or_else(|_| "your-api-key".to_string()),
    "http://127.0.0.1:8080/v1/chat/completions".to_string(), // 修改此处
),
```

### 修改系统提示词

编辑 [src/data/llm.rs](file:///home/sheneternity/Work/Project/sheneternity-agent/src/data/llm.rs#L34-L37) 中的系统提示：

```rust
history: Arc::new(RwLock::new(vec![Message {
    role: "system".to_string(),
    content: "你是一个编程助手".to_string(), // 修改此处
}])),
```

### 调整历史消息数量

编辑 [src/data/llm.rs](file:///home/sheneternity/Work/Project/sheneternity-agent/src/data/llm.rs#L38) 中的 `max_history`：

```rust
max_history: 12, // 修改此处
```

## 🎯 界面布局

```
┌─────────────────────────────────────────────────────┐
│   模式显示     │                                     │ 
├──────────────┬                                      │
│ 当前聊天列表   │                                      │
│              │         聊天窗口                      │
│              │                                      │
├──────────────┤                                      │
│ 历史聊天列表   │                                      │
│              ├──────────────────────────────────────┤
│              │           输入框                      │
└──────────────┴──────────────────────────────────────┘
```

## 📄 许可证

[LICENSE](LICENSE)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！
