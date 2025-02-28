use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use rust_i18n::t;
use std::time::Instant;

use crate::components::{Defense, DefensePerSecond, Gold, GoldPerSecond, LastClick};

pub struct TimingData<'a> {
    pub last_click: &'a LastClick,
    pub click_cooldown: f32,
    pub next_event_cooldown: f32,
}

pub struct StatsData<'a> {
    pub gold: &'a Gold,
    pub gold_ps: &'a GoldPerSecond,
    pub defense: &'a Defense,
    pub dps: &'a DefensePerSecond,
}

pub fn render_stats(frame: &mut Frame, area: Rect, stats: StatsData, timing: TimingData) {
    let stats_block = Block::default()
        .title(Line::from(t!("game.stats.title")).style(Style::default().fg(Color::Yellow).bold()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let now = Instant::now();
    let elapsed = now.duration_since(timing.last_click.0).as_secs_f32();
    let cooldown_remaining = if elapsed >= timing.click_cooldown {
        0.0
    } else {
        timing.click_cooldown - elapsed
    };

    let stats_text = vec![
        Line::from(vec![
            Span::styled("💰 ", Style::default().fg(Color::Yellow)),
            Span::raw(t!("game.stats.gold", amount = stats.gold.0)),
        ]),
        Line::from(vec![
            Span::styled("💎 ", Style::default().fg(Color::Cyan)),
            Span::raw(t!("game.stats.gold_per_second", amount = stats.gold_ps.0)),
        ]),
        Line::from(vec![
            Span::styled("🛡️ ", Style::default().fg(Color::Blue)),
            Span::raw(t!("game.stats.defense", amount = stats.defense.0)),
        ]),
        Line::from(vec![
            Span::styled("⚔️ ", Style::default().fg(Color::Red)),
            Span::raw(t!("game.stats.defense_per_second", amount = stats.dps.0)),
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
                time = format!("{:.1}", timing.next_event_cooldown)
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
