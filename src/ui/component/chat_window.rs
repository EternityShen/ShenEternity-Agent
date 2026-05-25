use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::data::{app::App, llm::DisplayItem};

pub fn draw(frame: &mut Frame, rect: Rect, app: &mut App) -> anyhow::Result<()> {
    let lines = build_lines(app);

    if app.can_auto_scroll {
        let total_lines = lines.len();
        if total_lines >= rect.height as usize {
            app.scroll = total_lines.saturating_sub(rect.height as usize) as u16 + 4;
        } else {
            app.scroll = total_lines.saturating_sub(rect.height as usize) as u16;
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Chat "))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightMagenta))
                .merge_borders(MergeStrategy::Exact),
        );

    frame.render_widget(paragraph, rect);
    Ok(())
}

fn build_lines(app: &App) -> Vec<Line<'static>> {
    let display_items: Vec<DisplayItem> = app
        .llm_handle
        .history
        .read()
        .unwrap()
        .iter()
        .flat_map(|m| m.to_display_items())
        .collect();

    let mut lines = Vec::new();

    for item in display_items {
        match item {
            DisplayItem::User(c) => {
                lines.push(Line::from(vec![Span::styled(
                    "● User ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::ITALIC),
                )]));

                lines.extend(c.lines().map(|l| {
                    Line::from(Span::styled(
                        l.to_string(),
                        Style::default().fg(Color::Gray),
                    ))
                }));
                lines.push(Line::from("\n"));
            }

            DisplayItem::Think(t) => {
                lines.push(Line::from(vec![Span::styled(
                    "* Thinking ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::ITALIC),
                )]));

                lines.extend(t.lines().map(|l| {
                    Line::from(Span::styled(
                        l.to_string(),
                        Style::default().fg(Color::Gray),
                    ))
                }));
                lines.push(Line::from("\n"));
            }

            DisplayItem::Chat(c) => {
                lines.push(Line::from(vec![Span::styled(
                    "| LLM ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC),
                )]));

                lines.extend(c.lines().map(|l| {
                    Line::from(Span::styled(
                        l.to_string(),
                        Style::default().fg(Color::Gray),
                    ))
                }));
                lines.push(Line::from("\n"));
            }

            DisplayItem::ToolCall(tc) => {
                lines.push(Line::from(Span::styled(
                    format!("⚙ {}", tc),
                    Style::default().fg(Color::Yellow),
                )));
                lines.push(Line::from("\n"));
            }
            DisplayItem::ToolResult(r) => {
                lines.push(Line::from(Span::styled(
                    format!("✓ {}", r),
                    Style::default().fg(Color::Cyan),
                )));
                lines.push(Line::from("\n"));
            }

            DisplayItem::System(s) => {
                lines.push(Line::from(vec![Span::styled(
                    "● System ",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                )]));

                lines.extend(s.lines().map(|l| {
                    Line::from(Span::styled(
                        l.to_string(),
                        Style::default().fg(Color::Gray),
                    ))
                }));
                lines.push(Line::from("\n"));
            }
        }
    }

    lines
}
