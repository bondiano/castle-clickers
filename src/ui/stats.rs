use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use rust_i18n::t;
use std::time::Instant;

use crate::components::{Defense, DefensePerSecond, Gold, GoldPerSecond, LastClick};

pub fn render_stats(
    frame: &mut Frame,
    area: Rect,
    gold: &Gold,
    gold_ps: &GoldPerSecond,
    defense: &Defense,
    dps: &DefensePerSecond,
    last_click: &LastClick,
    click_cooldown: f32,
    next_event_cooldown: f32,
) {
    let stats_block = Block::default()
        .title(Line::from(t!("game.stats.title")).style(Style::default().fg(Color::Yellow).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let now = Instant::now();
    let elapsed = now.duration_since(last_click.0).as_secs_f32();
    let cooldown_remaining = if elapsed >= click_cooldown {
        0.0
    } else {
        click_cooldown - elapsed
    };

    let stats_text = vec![
        Line::from(vec![
            Span::styled("💰 ", Style::default().fg(Color::Yellow)),
            Span::raw(t!("game.stats.gold", amount = gold.0)),
        ]),
        Line::from(vec![
            Span::styled("💎 ", Style::default().fg(Color::Cyan)),
            Span::raw(t!("game.stats.gold_per_second", amount = gold_ps.0)),
        ]),
        Line::from(vec![
            Span::styled("🛡️ ", Style::default().fg(Color::Blue)),
            Span::raw(t!("game.stats.defense", amount = defense.0)),
        ]),
        Line::from(vec![
            Span::styled("⚔️ ", Style::default().fg(Color::Red)),
            Span::raw(t!("game.stats.defense_per_second", amount = dps.0)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⏳ ", Style::default().fg(Color::Magenta)),
            Span::raw(t!(
                "game.stats.click_cooldown",
                time = format!("{:.1}", cooldown_remaining)
            )),
        ]),
        Line::from(vec![
            Span::styled("⏳ ", Style::default().fg(Color::Magenta)),
            Span::raw(t!(
                "game.stats.next_event",
                time = format!("{:.1}", next_event_cooldown)
            )),
        ]),
        Line::from(""),
        Line::styled(t!("game.controls.space"), Style::default().fg(Color::Gray)),
        Line::styled(t!("game.controls.arrows"), Style::default().fg(Color::Gray)),
        Line::styled(t!("game.controls.enter"), Style::default().fg(Color::Gray)),
        Line::styled(t!("game.controls.quit"), Style::default().fg(Color::Gray)),
    ];

    let stats = Paragraph::new(stats_text)
        .block(stats_block)
        .alignment(Alignment::Left);

    frame.render_widget(stats, area);
}
