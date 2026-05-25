use crate::data::llm::LLM;

pub struct App {
    pub llm_handle: LLM,
    pub input: String,
    pub is_quit: bool,
    pub scroll: u16,
    pub can_auto_scroll: bool,
    pub curror: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            llm_handle: LLM::new(
                std::env::var("API_KEY")
                    .unwrap_or_else(|_| "sk-6cb0c095a630453aa9b6194f38dadf07".to_string()),
                "https://api.deepseek.com/v1/chat/completions".to_string(),
            ),
            input: String::new(),
            is_quit: false,
            scroll: 0,
            can_auto_scroll: true,
            curror: false,
        }
    }
}

impl App {
    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn set_auto_scroll(&mut self) {
        self.can_auto_scroll = !self.can_auto_scroll;
    }

    pub fn scroll_down(&mut self) {
        self.scroll = (self.scroll + 1).min(200)
    }
}
