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
use rust_i18n::t;
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
    while world.query::<&components::GameRunning>().single(&world).0 {
        schedule.run(&mut world);
        render_game(&mut terminal, &mut world)?;
        input::handle_input(&mut world, &mut upgrade_schedule)?;
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    let (defense, max_defense) = world
        .query::<(&components::Defense, &components::MaxDefense)>()
        .single(&world);
    println!(
        "{}",
        t!(
            "game.game_over",
            defense = defense.0,
            max_defense = max_defense.0
        )
    );

    Ok(())
}

fn setup_world(config: game::Config) -> World {
    let mut world = World::new();

    world.spawn(GameBundle {
        gold: components::Gold(100),
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
        upgrades: components::Upgrades::default(),
        game_running: components::GameRunning(true),
        config,
    });

    world
}

fn setup_schedules() -> (Schedule, Schedule) {
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            systems::update_gold_system,
            systems::handle_events_system,
            systems::update_defense_by_per_second_system,
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
            ) = entity;

            let midas_level = upgrades.get_count(&AvailableUpgrades::MidasHand(0));
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
            ui::upgrades::render_upgrades(&mut frame, upgrades_area, upgrades, selected, config);
            ui::events::render_event(&mut frame, event_area, event);
        })
        .map(|_| ())
}
