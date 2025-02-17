use bevy_ecs::prelude::*;
use std::time::Instant;

use crate::game::Config;

#[derive(Component)]
pub struct Gold(pub u64);

#[derive(Component)]
pub struct GoldPerSecond(pub u64);

#[derive(Component)]
pub struct Defense(pub u64);

#[derive(Component)]
pub struct DefensePerSecond(pub u64);

#[derive(Component)]
pub struct MaxDefense(pub u64);

#[derive(Component, PartialEq, Copy, Clone)]
pub enum AvailableUpgrades {
    Catapult,
    Archer,
    Warrior,
    Officer,
    OilReserve,
    TradeHall(u32),
    MidasHand(u32),
}

impl AvailableUpgrades {
    pub fn cost(&self, config: &Config) -> u64 {
        match self {
            AvailableUpgrades::Catapult => config.upgrades.catapult.cost,
            AvailableUpgrades::Archer => config.upgrades.archer.cost,
            AvailableUpgrades::Warrior => config.upgrades.warrior.cost,
            AvailableUpgrades::Officer => config.upgrades.officer.cost,
            AvailableUpgrades::OilReserve => config.upgrades.oil.cost,
            AvailableUpgrades::TradeHall(level) => {
                let level_index = *level as usize;
                if level_index >= config.trade_hall.levels.len() {
                    return 0;
                }
                (config.trade_hall.base_cost as f32
                    * config.trade_hall.levels[level_index].cost_multiplier) as u64
            }
            AvailableUpgrades::MidasHand(level) => {
                let level_index = *level as usize;
                if level_index >= config.midas_hand.levels.len() {
                    return 0;
                }
                (config.midas_hand.base_cost as f32
                    * config.midas_hand.levels[level_index].cost_multiplier) as u64
            }
        }
    }
}

#[derive(Component)]
pub struct SelectedUpgrade(pub AvailableUpgrades);

#[derive(Component)]
pub struct LastTick(pub Instant);

#[derive(Component)]
pub struct LastEventCheck(pub Instant);

#[derive(Component)]
pub struct LastClick(pub Instant);

#[derive(Component)]
pub struct EventMessage {
    pub message: String,
    pub timestamp: Instant,
}

#[derive(Component)]
pub struct GameRunning(pub bool);

#[derive(Component)]
pub struct Upgrades(pub [AvailableUpgrades; 7]);

impl Default for Upgrades {
    fn default() -> Self {
        Self([
            AvailableUpgrades::Catapult,
            AvailableUpgrades::Archer,
            AvailableUpgrades::Warrior,
            AvailableUpgrades::Officer,
            AvailableUpgrades::OilReserve,
            AvailableUpgrades::TradeHall(0),
            AvailableUpgrades::MidasHand(0),
        ])
    }
}

impl Upgrades {
    pub fn get_count(&self, upgrade: &AvailableUpgrades) -> u32 {
        let index = self
            .0
            .iter()
            .position(|u| match (u, upgrade) {
                (AvailableUpgrades::TradeHall(_), AvailableUpgrades::TradeHall(_)) => true,
                (AvailableUpgrades::MidasHand(_), AvailableUpgrades::MidasHand(_)) => true,
                (a, b) => a == b,
            })
            .unwrap_or(0);

        match self.0[index] {
            AvailableUpgrades::TradeHall(level) => level,
            AvailableUpgrades::MidasHand(level) => level,
            _ => 0,
        }
    }

    pub fn increment(&mut self, upgrade: &AvailableUpgrades) {
        let index = self
            .0
            .iter()
            .position(|u| match (u, upgrade) {
                (AvailableUpgrades::TradeHall(_), AvailableUpgrades::TradeHall(_)) => true,
                (AvailableUpgrades::MidasHand(_), AvailableUpgrades::MidasHand(_)) => true,
                (a, b) => a == b,
            })
            .unwrap_or(0);

        match self.0[index] {
            AvailableUpgrades::TradeHall(level) => {
                self.0[index] = AvailableUpgrades::TradeHall(level + 1);
            }
            AvailableUpgrades::MidasHand(level) => {
                self.0[index] = AvailableUpgrades::MidasHand(level + 1);
            }
            _ => {}
        }
    }

    pub fn apply_defense(&self, defense: &mut Defense, config: &Config) {
        defense.0 += match self.0[0] {
            AvailableUpgrades::Catapult => config.upgrades.catapult.defense,
            AvailableUpgrades::Archer => config.upgrades.archer.defense,
            AvailableUpgrades::Warrior => config.upgrades.warrior.defense,
            AvailableUpgrades::Officer => config.upgrades.officer.defense,
            _ => 0,
        };
    }
}
