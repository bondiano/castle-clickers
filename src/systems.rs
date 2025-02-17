use bevy_ecs::prelude::*;
use rust_i18n::t;
use std::time::{Duration, Instant};

use crate::{
    components::*,
    game::{self, Config},
};

pub fn update_defense_by_per_second_system(
    mut query: Query<(&Upgrades, &mut Defense, &mut LastTick, &Config)>,
) {
    for (upgrades, mut defense, mut last_tick, config) in query.iter_mut() {
        let now = Instant::now();
        let elapsed = now.duration_since(last_tick.0);

        if elapsed >= Duration::from_secs(1) {
            defense.0 += game::calculate_defense_per_second(upgrades, config);
            last_tick.0 = now;
        }
    }
}

pub fn update_gold_system(
    mut query: Query<(
        &mut Gold,
        &mut GoldPerSecond,
        &mut LastTick,
        &Upgrades,
        &Config,
    )>,
) {
    for (mut gold, mut gold_ps, mut last_tick, upgrades, config) in query.iter_mut() {
        let now = Instant::now();
        let elapsed = now.duration_since(last_tick.0);

        if elapsed >= Duration::from_secs(1) {
            gold_ps.0 = game::calculate_gold_per_second(upgrades, config);
            gold.0 += gold_ps.0;
            last_tick.0 = now;
        }
    }
}

pub fn handle_events_system(
    mut query: Query<(
        &mut Defense,
        &mut Gold,
        &mut LastEventCheck,
        &mut EventMessage,
        &mut GameRunning,
        &Config,
    )>,
) {
    for (
        mut defense,
        mut gold,
        mut last_event_check,
        mut event_message,
        mut game_running,
        config,
    ) in query.iter_mut()
    {
        let now = Instant::now();
        let next_event_at = last_event_check.0
            + Duration::from_millis((config.events.cooldown * 1000.0).round() as u64);
        let hide_message_at = next_event_at - Duration::from_secs(1);

        // Clear old messages
        if now >= hide_message_at {
            event_message.message.clear();
        }

        if now >= next_event_at {
            let event = game::generate_random_event(config);
            match event {
                game::GameEvent::BanditRaid {
                    defense_loss,
                    message,
                }
                | game::GameEvent::SiegeCatapults {
                    defense_loss,
                    message,
                }
                | game::GameEvent::Sabotage {
                    defense_loss,
                    message,
                } => {
                    if defense.0 >= defense_loss {
                        defense.0 -= defense_loss;
                        event_message.message = t!(
                            "game.events.defense_loss",
                            message = message,
                            amount = defense_loss
                        )
                        .to_string();
                    } else {
                        defense.0 = 0;
                        event_message.message =
                            t!("game.events.castle_fall", message = message).to_string();
                        game_running.0 = false;
                    }
                }
                game::GameEvent::TradeOffer { gold_gain, message } => {
                    gold.0 += gold_gain;
                    event_message.message = t!(
                        "game.events.trade_gain",
                        message = message,
                        amount = gold_gain
                    )
                    .to_string();
                }
                game::GameEvent::Nothing => {
                    event_message.message = t!("game.events.nothing").to_string();
                }
            }
            event_message.timestamp = now;
            last_event_check.0 = now;
        }
    }
}

pub fn handle_upgrade_system(
    mut query: Query<(
        &mut Gold,
        &mut Defense,
        &mut Upgrades,
        &SelectedUpgrade,
        &Config,
    )>,
) {
    for (mut gold, mut defense, mut upgrades, selected, config) in query.iter_mut() {
        let cost = selected.0.cost(config);

        if !game::can_afford_upgrade(gold.0, cost) {
            continue;
        }

        gold.0 -= cost;
        upgrades.increment(&selected.0);
        upgrades.apply_defense(&mut defense, &config);
    }
}
