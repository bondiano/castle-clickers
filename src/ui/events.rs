use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use rust_i18n::t;

use crate::components::EventMessage;

pub fn render_event(frame: &mut Frame, area: Rect, event: &EventMessage) {
    if !event.message.is_empty() {
        let event_block = Block::default()
            .title(
                Line::from(t!("game.events.title")).style(Style::default().fg(Color::Red).bold()),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));

        let event_text = Paragraph::new(
            Line::from(event.message.as_str()).style(Style::default().fg(Color::Red).bold()),
        )
        .block(event_block)
        .alignment(Alignment::Center);

        frame.render_widget(event_text, area);
    }
}
