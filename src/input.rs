use bevy_ecs::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};

use crate::{
    components::{AvailableUpgrades, GameRunning, Gold, LastClick, SelectedUpgrade, Upgrades},
    game::{self, Config},
};

pub fn handle_input(world: &mut World, upgrade_schedule: &mut Schedule) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    let mut running_query = world.query::<&mut GameRunning>();
                    running_query.single_mut(world).0 = false;
                }
                KeyCode::Char(' ') if key.kind == KeyEventKind::Press => {
                    let mut click_query =
                        world.query::<(&mut Gold, &mut LastClick, &Config, &Upgrades)>();

                    if let Ok((mut gold, mut last_click, config, upgrades)) =
                        click_query.get_single_mut(world)
                    {
                        let now = Instant::now();
                        let midas_hand = upgrades.get_count(&AvailableUpgrades::MidasHand(0));
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
