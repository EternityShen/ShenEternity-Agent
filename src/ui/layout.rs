use ratatui::{
    Frame,
    layout::{Constraint, Layout, Spacing},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders},
};

use crate::{
    data::app::App,
    ui::component::{chat_window, input_bar},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .title("ShenEternity-Agent")
        .title_style(Style::default().fg(Color::LightBlue));

    frame.render_widget(block, frame.area());

    let chunks_h = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Min(30),
        Constraint::Length(1),
    ])
    .spacing(1)
    .split(frame.area());

    let chunks_v = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(30),
        Constraint::Length(1),
    ])
    .spacing(0)
    .split(chunks_h[1]);

    let chunks_v = Layout::vertical([Constraint::Min(30), Constraint::Length(3)])
        .spacing(Spacing::Overlap(1))
        .split(chunks_v[1]);

    let top_chunk = chunks_v[0];
    let bottom_chunk = chunks_v[1];

    let top_chunks_h = Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
        .spacing(Spacing::Overlap(1))
        .split(top_chunk);

    let bottom_chunks_h =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .spacing(Spacing::Overlap(1))
            .split(bottom_chunk);

    let top_left = top_chunks_h[0];
    let top_right = top_chunks_h[1];

    let bottom_left = bottom_chunks_h[0];
    let bottom_right = bottom_chunks_h[1];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .merge_borders(MergeStrategy::Exact);

    frame.render_widget(block.clone(), top_right);
    frame.render_widget(block.clone(), bottom_right);

    input_bar::draw(frame, bottom_left, app).unwrap();
    chat_window::draw(frame, top_left, app).unwrap();
}
