use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use rust_i18n::t;

use crate::components::{Defense, MaxDefense};

pub fn render_game_over(
    frame: &mut Frame,
    area: Rect,
    defense: &Defense,
    max_defense: &MaxDefense,
) {
    let game_over_block = Block::default()
        .title(Line::from(t!("game.game_over.title")).style(Style::default().fg(Color::Red).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let game_over_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("game.game_over.message"),
            Style::default().fg(Color::Red).bold(),
        )]),
        Line::from(""),
        Line::from(vec![Span::raw(t!(
            "game.game_over.score",
            defense = defense.0,
            max_defense = max_defense.0
        ))]),
        Line::from(""),
        Line::from(""),
        Line::styled(
            t!("game.game_over.restart"),
            Style::default().fg(Color::Green).bold(),
        ),
        Line::styled(
            t!("game.game_over.exit"),
            Style::default().fg(Color::Yellow).bold(),
        ),
    ];

    let paragraph = Paragraph::new(game_over_text)
        .block(game_over_block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
