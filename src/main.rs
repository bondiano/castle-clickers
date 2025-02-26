mod components;
mod game;
mod input;
mod systems;
mod ui;

use bevy_ecs::prelude::*;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, Terminal};
use std::{io, time::Instant};

use crate::components::AvailableUpgrades;

rust_i18n::i18n!("locales");

#[derive(Bundle)]
struct GameBundle {
    gold: components::Gold,
    gold_ps: components::GoldPerSecond,
    defense: components::Defense,
    max_defense: components::MaxDefense,
    defense_ps: components::DefensePerSecond,
    upgrades: components::Upgrades,
    last_tick: components::LastTick,
    last_event_check: components::LastEventCheck,
    last_click: components::LastClick,
    event_message: components::EventMessage,
    selected_upgrade: components::SelectedUpgrade,
    game_running: components::GameRunning,
    game_state: components::GameState,
    bought_upgrades: components::BoughtUpgrades,
    config: game::Config,
}

fn main() -> io::Result<()> {
    rust_i18n::set_locale("ru");

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let config = game::Config::load().expect("Failed to load game config");
    let mut world = setup_world(config);
    let (mut schedule, mut upgrade_schedule) = setup_schedules();

    // Game loop
    loop {
        let game_state = world.query::<&components::GameState>().single(&world);

        match *game_state {
            components::GameState::Playing => {
                schedule.run(&mut world);
                render_game(&mut terminal, &mut world)?;
                input::handle_input(&mut world, &mut upgrade_schedule)?;
            }
            components::GameState::GameOver => {
                render_game_over(&mut terminal, &mut world)?;
                input::handle_game_over_input(&mut world)?;
            }
            components::GameState::Exiting => {
                break;
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn setup_world(config: game::Config) -> World {
    let mut world = World::new();

    world.spawn(GameBundle {
        gold: components::Gold(200),
        gold_ps: components::GoldPerSecond(0),
        defense: components::Defense(100),
        max_defense: components::MaxDefense(100),
        defense_ps: components::DefensePerSecond(0),
        last_tick: components::LastTick(Instant::now()),
        last_event_check: components::LastEventCheck(Instant::now()),
        last_click: components::LastClick(Instant::now()),
        event_message: components::EventMessage {
            message: String::new(),
            timestamp: Instant::now(),
        },
        selected_upgrade: components::SelectedUpgrade(AvailableUpgrades::Catapult),
        bought_upgrades: components::BoughtUpgrades::default(),
        upgrades: components::Upgrades::default(),
        game_running: components::GameRunning(true),
        game_state: components::GameState::default(),
        config,
    });

    world
}

fn setup_schedules() -> (Schedule, Schedule) {
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            systems::update_per_second_system,
            systems::handle_events_system,
        )
            .chain(),
    );

    let mut upgrade_schedule = Schedule::default();
    upgrade_schedule.add_systems((systems::handle_upgrade_system,).chain());

    (schedule, upgrade_schedule)
}

fn render_game(terminal: &mut Terminal<impl Backend>, world: &mut World) -> io::Result<()> {
    let mut query = world.query::<(
        &components::Gold,
        &components::GoldPerSecond,
        &components::Defense,
        &components::DefensePerSecond,
        &components::Upgrades,
        &components::SelectedUpgrade,
        &game::Config,
        &components::EventMessage,
        &components::LastClick,
        &components::LastEventCheck,
        &components::BoughtUpgrades,
    )>();
    let entity = query.get_single(world).unwrap();

    terminal
        .draw(|mut frame| {
            let (event_area, stats_area) = ui::create_layout(frame.area());
            let upgrades_area = Rect::new(
                stats_area.x + stats_area.width,
                stats_area.y,
                stats_area.width,
                stats_area.height,
            );

            let (
                gold,
                gold_ps,
                defense,
                dps,
                upgrades,
                selected,
                config,
                event,
                last_click,
                last_event_check,
                bought_upgrades,
            ) = entity;

            let midas_level = bought_upgrades.get_count(&AvailableUpgrades::MidasHand);
            let click_cooldown = game::calculate_click_cooldown(midas_level, config);
            let next_event_cooldown =
                game::calculate_next_event_cooldown(last_event_check.0, config);
            ui::stats::render_stats(
                &mut frame,
                stats_area,
                gold,
                gold_ps,
                defense,
                dps,
                last_click,
                click_cooldown,
                next_event_cooldown,
            );
            ui::upgrades::render_upgrades(
                &mut frame,
                upgrades_area,
                upgrades,
                selected,
                bought_upgrades,
                config,
            );
            ui::events::render_event(&mut frame, event_area, event);
        })
        .map(|_| ())
}

fn render_game_over(terminal: &mut Terminal<impl Backend>, world: &mut World) -> io::Result<()> {
    let mut query = world.query::<(&components::Defense, &components::MaxDefense)>();
    let (defense, max_defense) = query.single_mut(world);

    terminal
        .draw(|mut frame| {
            let area = ui::create_game_over_layout(frame.area());
            ui::game_over::render_game_over(&mut frame, area, defense, max_defense);
        })
        .map(|_| ())
}
