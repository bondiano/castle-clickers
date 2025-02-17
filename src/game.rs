use bevy_ecs::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::{
    fs,
    time::{Duration, Instant},
};

use crate::components;

#[derive(Debug, Deserialize, Component)]
pub struct Config {
    pub upgrades: ConfigUpgrades,
    pub events: Events,
    pub trade: Trade,
    pub trade_hall: TradeHallConfig,
    pub midas_hand: MidasHandConfig,
    pub click: ClickConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigUpgrades {
    pub catapult: UpgradeConfig,
    pub archer: UpgradeConfig,
    pub warrior: UpgradeConfig,
    pub officer: UpgradeConfig,
    pub oil: UpgradeConfig,
}

#[derive(Debug, Deserialize)]
pub struct UpgradeConfig {
    pub cost: u64,
    pub defense: u64,
    pub defense_per_second: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeHallConfig {
    pub base_cost: u64,
    pub base_gold_per_second: u64,
    pub levels: Vec<TradeHallLevel>,
}

#[derive(Debug, Deserialize)]
pub struct TradeHallLevel {
    pub cost_multiplier: f32,
    pub gold_multiplier: f32,
}

#[derive(Debug, Deserialize)]
pub struct MidasHandConfig {
    pub required_trade_hall_level: u32,
    pub base_cost: u64,
    pub cooldown_reduction: f32,
    pub levels: Vec<MidasHandLevel>,
}

#[derive(Debug, Deserialize)]
pub struct MidasHandLevel {
    pub cost_multiplier: f32,
}

#[derive(Debug, Deserialize)]
pub struct ClickConfig {
    pub base_cooldown: f32,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    pub bandit_raid: EventConfig,
    pub siege_catapults: EventConfig,
    pub sabotage: EventConfig,
    pub cooldown: f32,
}

#[derive(Debug, Deserialize)]
pub struct EventConfig {
    pub min_damage: u64,
    pub max_damage: u64,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub offer: TradeConfig,
}

#[derive(Debug, Deserialize)]
pub struct TradeConfig {
    pub min_gain: u64,
    pub max_gain: u64,
    pub message: String,
}

#[derive(Debug)]
pub enum GameEvent {
    BanditRaid { defense_loss: u64, message: String },
    SiegeCatapults { defense_loss: u64, message: String },
    Sabotage { defense_loss: u64, message: String },
    TradeOffer { gold_gain: u64, message: String },
    Nothing,
}

pub fn generate_random_event(config: &Config) -> GameEvent {
    let mut rng = rand::rng();

    if rng.random_ratio(1, 20) {
        match rng.random_range(0..4) {
            0 => GameEvent::BanditRaid {
                defense_loss: rng.random_range(
                    config.events.bandit_raid.min_damage..=config.events.bandit_raid.max_damage,
                ),
                message: config.events.bandit_raid.message.clone(),
            },
            1 => GameEvent::SiegeCatapults {
                defense_loss: rng.random_range(
                    config.events.siege_catapults.min_damage
                        ..=config.events.siege_catapults.max_damage,
                ),
                message: config.events.siege_catapults.message.clone(),
            },
            2 => GameEvent::Sabotage {
                defense_loss: rng.random_range(
                    config.events.sabotage.min_damage..=config.events.sabotage.max_damage,
                ),
                message: config.events.sabotage.message.clone(),
            },
            3 => GameEvent::TradeOffer {
                gold_gain: rng
                    .random_range(config.trade.offer.min_gain..=config.trade.offer.max_gain),
                message: config.trade.offer.message.clone(),
            },
            _ => GameEvent::Nothing,
        }
    } else {
        GameEvent::Nothing
    }
}

pub fn calculate_defense_per_second(upgrades: &components::Upgrades, config: &Config) -> u64 {
    upgrades.0.iter().fold(0, |acc, upgrade| match upgrade {
        components::AvailableUpgrades::Officer => acc + config.upgrades.officer.defense_per_second,
        _ => acc,
    })
}

pub fn calculate_gold_per_second(upgrades: &components::Upgrades, config: &Config) -> u64 {
    upgrades.0.iter().fold(0, |acc, upgrade| match upgrade {
        components::AvailableUpgrades::TradeHall(level) => {
            if *level == 0 {
                return 0;
            }
            let level_index = (*level - 1) as usize;
            let level = &config.trade_hall.levels[level_index];
            acc + (config.trade_hall.base_gold_per_second as f32 * level.gold_multiplier) as u64
        }
        _ => acc,
    })
}

pub fn calculate_click_cooldown(midas_level: u32, config: &Config) -> f32 {
    let base_cooldown = config.click.base_cooldown;
    let reduction = if midas_level > 0 {
        let level_index = (midas_level - 1) as usize;
        if level_index < config.midas_hand.levels.len() {
            midas_level as f32 * config.midas_hand.cooldown_reduction
        } else {
            0.0
        }
    } else {
        0.0
    };
    base_cooldown - reduction
}

pub fn can_afford_upgrade(gold: u64, cost: u64) -> bool {
    gold >= cost
}

pub fn calculate_next_event_cooldown(last_event_check: Instant, config: &Config) -> f32 {
    let now = Instant::now();
    let next_event_at =
        last_event_check + Duration::from_millis((config.events.cooldown * 1000.0).round() as u64);

    next_event_at.duration_since(now).as_secs_f32()
}
