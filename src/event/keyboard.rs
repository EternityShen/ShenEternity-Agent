use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::data::app::App;

pub async fn event(app: &mut App) {
    if event::poll(Duration::from_millis(200)).unwrap()
        && let Event::Key(key) = event::read().unwrap()
    {
        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.is_quit = true
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.llm_handle.abort();
            }
            KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let msg = app.llm_handle.withdraw();
                app.input = msg;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Enter => {
                if app.input.is_empty() {
                } else {
                    let input = app.input.clone();
                    let mut llm = app.llm_handle.clone();
                    app.input.clear();
                    tokio::spawn(async move {
                        llm.chat(input).await.unwrap();
                    });
                }
            }
            KeyCode::Up => {
                app.scroll_up();
            }
            KeyCode::Tab => {
                app.set_auto_scroll();
            }
            KeyCode::Down => {
                app.scroll_down();
            }
            _ => {}
        }
    }
}
