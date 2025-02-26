use bevy_ecs::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};

use crate::{
    components::{
        AvailableUpgrades, BoughtUpgrades, Defense, DefensePerSecond, EventMessage, GameRunning,
        GameState, Gold, GoldPerSecond, LastClick, LastEventCheck, MaxDefense, SelectedUpgrade,
        Upgrades,
    },
    game::{self, Config},
};

pub fn handle_input(world: &mut World, upgrade_schedule: &mut Schedule) -> std::io::Result<()> {
    // First check if we're in Playing state
    let game_state = world.query::<&GameState>().single(world);
    if *game_state != GameState::Playing {
        return Ok(());
    }

    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    let mut state_query = world.query::<(&mut GameRunning, &mut GameState)>();
                    let (mut game_running, mut game_state) = state_query.single_mut(world);
                    game_running.0 = false;
                    *game_state = GameState::Exiting;
                }
                KeyCode::Char(' ') if key.kind == KeyEventKind::Press => {
                    let mut click_query =
                        world.query::<(&mut Gold, &mut LastClick, &Config, &BoughtUpgrades)>();

                    if let Ok((mut gold, mut last_click, config, bought_upgrades)) =
                        click_query.get_single_mut(world)
                    {
                        let now = Instant::now();
                        let midas_hand = bought_upgrades.get_count(&AvailableUpgrades::MidasHand);
                        let cooldown = game::calculate_click_cooldown(midas_hand, config);
                        if now.duration_since(last_click.0).as_secs_f32() >= cooldown {
                            gold.0 += 1;
                            last_click.0 = now;
                        }
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let mut query = world.query::<(&Upgrades, &mut SelectedUpgrade)>();
                    let (upgrades, mut selected) = query.single_mut(world);

                    let current_index = upgrades.0.iter().position(|&u| u == selected.0).unwrap();
                    if current_index > 0 {
                        selected.0 = upgrades.0[current_index - 1];
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let mut query = world.query::<(&Upgrades, &mut SelectedUpgrade)>();
                    let (upgrades, mut selected) = query.single_mut(world);

                    let current_index = upgrades.0.iter().position(|&u| u == selected.0).unwrap();
                    if current_index < upgrades.0.len() - 1 {
                        selected.0 = upgrades.0[current_index + 1];
                    }
                }
                KeyCode::Enter => {
                    upgrade_schedule.run(world);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn handle_game_over_input(world: &mut World) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        let mut state_query = world.query::<&mut GameState>();
                        *state_query.single_mut(world) = GameState::Exiting;
                    }
                    KeyCode::Enter => {
                        // Reset the game state for a new game
                        reset_game(world);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn reset_game(world: &mut World) {
    let mut query = world.query::<(
        &mut Gold,
        &mut GameState,
        &mut GameRunning,
        &mut LastClick,
        &mut LastEventCheck,
        &mut Defense,
        &mut MaxDefense,
        &mut GoldPerSecond,
        &mut DefensePerSecond,
        &mut EventMessage,
        &mut Upgrades,
        &mut SelectedUpgrade,
    )>();

    let (
        mut gold,
        mut game_state,
        mut game_running,
        mut last_click,
        mut last_event_check,
        mut defense,
        mut max_defense,
        mut gold_ps,
        mut defense_ps,
        mut event_message,
        mut upgrades,
        mut selected_upgrade,
    ) = query.single_mut(world);

    // Reset to initial game state
    *gold = Gold(100);
    *game_state = GameState::Playing;
    *game_running = GameRunning(true);
    *last_click = LastClick(Instant::now());
    *last_event_check = LastEventCheck(Instant::now());
    *defense = Defense(100);
    *max_defense = MaxDefense(100);
    *gold_ps = GoldPerSecond(0);
    *defense_ps = DefensePerSecond(0);
    *upgrades = Upgrades::default();
    *selected_upgrade = SelectedUpgrade(AvailableUpgrades::Catapult);
    event_message.message.clear();
}
