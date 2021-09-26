#![allow(clippy::type_complexity)]

pub mod components;
pub mod gamelog;
pub mod gui;
pub mod map;
pub mod map_builders;
pub mod render;
pub mod systems;

use std::collections::HashSet;
use std::path::Path;

use bevy_app::CoreStage;
use bevy_bracket_lib::BracketLibPlugin;
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

/// Width of the console window.
pub const CONSOLE_WIDTH: u32 = 80;
/// Height of the console window.
pub const CONSOLE_HEIGHT: u32 = 60;

/// Event that is emitted when input is received.
pub struct InputEvent;

fn main() {
    tracing_subscriber::fmt::init();

    /// Label for our rendering stage.
    static RENDER_STAGE: &str = "render";

    let font_path = Path::new("static/terminal_8x8.png");
    let font_path = font_path.canonicalize().unwrap();

    let mut bterm = BTermBuilder::new()
        .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, font_path.to_str().unwrap())
        .with_title("Tailarc")
        .with_font(font_path.to_str().unwrap(), 8, 8)
        .build()
        .unwrap();
    bterm.with_post_scanlines(true);

    bevy_app::App::build()
        .add_plugin(CorePlugin::default())
        .add_stage_after(
            CoreStage::Update,
            RENDER_STAGE,
            SystemStage::single_threaded(),
        )
        .add_plugin(BracketLibPlugin::new(bterm))
        .add_event::<InputEvent>()
        .add_startup_system(init.system())
        // Handle input first. Input is what triggers the game to update.
        .add_system(
            systems::input::player_input_system
                .system()
                .chain(systems::input::update_player_position_system.system())
                .label("input"),
        )
        // Run indexing systems after input to ensure that state is in sync.
        .add_system_set(
            SystemSet::new()
                .with_system(systems::visibility::visibility_system.system())
                .with_system(systems::map_indexing::map_indexing_system.system())
                .label("indexing")
                .after("input"),
        )
        // Run the rest of the game systems.
        .add_system(
            systems::monster_ai::monster_ai_system
                .system()
                .after("indexing"),
        )
        .add_system_to_stage(
            RENDER_STAGE,
            render::render
                .system()
                .chain(gui::render_ui_system.system()),
        )
        .run();
}

fn init(mut commands: Commands) {
    use components::{CombatStats, Player, PlayerBundle, Position, Renderable, Viewshed};

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        position: Position {
            x: (CONSOLE_WIDTH / 2) as i32,
            y: (CONSOLE_HEIGHT / 2) as i32,
        },
        renderable: Renderable {
            glyph: '@' as u16,
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        },
        viewshed: Viewshed {
            visible_tiles: HashSet::new(),
            range: 8,
            dirty: true,
        },
        combat_stats: CombatStats {
            hp: 30,
            max_hp: 30,
            defense: 2,
            power: 5,
        },
    });

    // Tile map resource.
    let map = map::Map::new_random(100, 100, true, &mut commands);
    commands.insert_resource(map);
    // Game log resource.
    commands.insert_resource(gamelog::GameLog {
        entries: vec!["Welcome to Tailarc!".to_string()],
    });

    tracing::info!("Finished initialization");
}
