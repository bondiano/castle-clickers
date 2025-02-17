use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use rust_i18n::t;

use crate::{
    components::{AvailableUpgrades, SelectedUpgrade, Upgrades},
    game::Config,
};

pub fn render_upgrades(
    frame: &mut Frame,
    area: Rect,
    upgrades: &Upgrades,
    selected: &SelectedUpgrade,
    config: &Config,
) {
    let upgrades_list: Vec<String> = upgrades
        .0
        .iter()
        .map(|upgrade| match upgrade {
            AvailableUpgrades::Catapult => format_upgrade_item(
                "🗡️",
                &config.upgrades.catapult.name,
                upgrade.cost(config),
                upgrades.get_count(upgrade),
            ),
            AvailableUpgrades::Archer => format_upgrade_item(
                "🏹",
                &config.upgrades.archer.name,
                upgrade.cost(config),
                upgrades.get_count(upgrade),
            ),
            AvailableUpgrades::Warrior => format_upgrade_item(
                "⚔️",
                &config.upgrades.warrior.name,
                upgrade.cost(config),
                upgrades.get_count(upgrade),
            ),
            AvailableUpgrades::Officer => format_upgrade_item(
                "👑",
                &config.upgrades.officer.name,
                upgrade.cost(config),
                upgrades.get_count(upgrade),
            ),
            AvailableUpgrades::OilReserve => format_upgrade_item(
                "🔥",
                &config.upgrades.oil.name,
                upgrade.cost(config),
                upgrades.get_count(upgrade),
            ),
            AvailableUpgrades::TradeHall(level) => {
                format!("🏛️ {}", t!("game.upgrades.trade_hall", level = level))
            }
            AvailableUpgrades::MidasHand(level) => {
                let trade_hall_level = upgrades.get_count(&AvailableUpgrades::TradeHall(0));
                if trade_hall_level >= config.midas_hand.required_trade_hall_level {
                    format!("✨ {}", t!("game.upgrades.midas_hand", level = level))
                } else {
                    t!("game.upgrades.midas_hand_locked").to_string()
                }
            }
        })
        .collect();

    let items: Vec<ListItem> = upgrades_list
        .iter()
        .enumerate()
        .map(|(i, upgrade)| {
            let style = if upgrades.0[i] == selected.0 {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(upgrade.as_str()).style(style)
        })
        .collect();

    let upgrades_list = List::new(items)
        .block(
            Block::default()
                .title(
                    Line::from(t!("game.upgrades.title"))
                        .style(Style::default().fg(Color::Green).bold()),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(upgrades_list, area);
}

fn format_upgrade_item(icon: &str, name: &str, cost: u64, amount: u32) -> String {
    format!(
        "{} {}",
        icon,
        t!(
            "game.upgrades.item_format",
            name = name,
            cost = cost,
            amount = amount
        )
    )
}
