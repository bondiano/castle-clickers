use bevy_ecs::prelude::*;
use std::{collections::HashMap, ops::AddAssign, time::Instant};

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

#[derive(Component, PartialEq, Copy, Clone, Eq, Hash)]
pub enum AvailableUpgrades {
    Catapult,
    Archer,
    Warrior,
    Officer,
    OilReserve,
    TradeHall,
    MidasHand,
}

impl AvailableUpgrades {
    pub fn cost(&self, config: &Config, bought_upgrades: &BoughtUpgrades) -> u64 {
        match self {
            AvailableUpgrades::Catapult => config.upgrades.catapult.cost,
            AvailableUpgrades::Archer => config.upgrades.archer.cost,
            AvailableUpgrades::Warrior => config.upgrades.warrior.cost,
            AvailableUpgrades::Officer => config.upgrades.officer.cost,
            AvailableUpgrades::OilReserve => config.upgrades.oil.cost,
            AvailableUpgrades::TradeHall => {
                let level = bought_upgrades.get_count(self);
                if level >= config.trade_hall.levels.len() as u32 {
                    return 0;
                }

                let level_index = level as usize;
                (config.trade_hall.base_cost as f32
                    * config.trade_hall.levels[level_index].cost_multiplier) as u64
            }
            AvailableUpgrades::MidasHand => {
                let level = bought_upgrades.get_count(self);
                if level >= config.midas_hand.levels.len() as u32 {
                    return 0;
                }

                let level_index = level as usize;
                (config.midas_hand.base_cost as f32
                    * config.midas_hand.levels[level_index].cost_multiplier) as u64
            }
        }
    }
}

#[derive(Component)]
pub struct SelectedUpgrade(pub AvailableUpgrades);

#[derive(Component, Default)]
pub struct BoughtUpgrades(pub HashMap<AvailableUpgrades, u32>);

impl BoughtUpgrades {
    pub fn get_count(&self, upgrade: &AvailableUpgrades) -> u32 {
        *self.0.get(upgrade).unwrap_or(&0)
    }

    pub fn increment(&mut self, upgrade: &AvailableUpgrades) {
        self.0.entry(*upgrade).or_insert(0).add_assign(1);
    }
}

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

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    GameOver,
    Exiting,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Playing
    }
}

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
            AvailableUpgrades::TradeHall,
            AvailableUpgrades::MidasHand,
        ])
    }
}

impl Upgrades {
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
