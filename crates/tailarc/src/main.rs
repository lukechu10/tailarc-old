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
use bevy_ecs::schedule::ShouldRun;
use bracket_lib::prelude::*;

/// Width of the console window.
pub const CONSOLE_WIDTH: u32 = 80;
/// Height of the console window.
pub const CONSOLE_HEIGHT: u32 = 60;

/// A resource that contains whether input has been received on the current frame.
pub struct InputState {
    has_input_on_frame: bool,
}

/// Only run if input has been received on the current frame.
pub fn run_if_input(input_state: Res<InputState>) -> ShouldRun {
    if input_state.has_input_on_frame {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub struct IsInitial(bool);

impl Default for IsInitial {
    fn default() -> Self {
        IsInitial(true)
    }
}

pub fn run_if_input_or_initial(
    input_state: Res<InputState>,
    mut is_initial: Local<IsInitial>,
) -> ShouldRun {
    if input_state.has_input_on_frame || is_initial.0 {
        *is_initial = IsInitial(false);
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    /// Label for our render stage.
    static RENDER_STAGE: &str = "render";

    #[cfg(feature = "trace")]
    tracing_subscriber::fmt::init();

    let mut bterm;
    if cfg!(not(target_arch = "wasm32")) {
        let font_path = Path::new("static/terminal_8x8.png");
        let font_path = font_path.canonicalize().unwrap();

        bterm = BTermBuilder::new()
            .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, font_path.to_str().unwrap())
            .with_title("Tailarc")
            .with_font(font_path.to_str().unwrap(), 8, 8)
            .build()
            .unwrap();
    } else {
        bterm = BTermBuilder::simple(CONSOLE_WIDTH, CONSOLE_HEIGHT)
            .unwrap()
            .with_title("Tailarc")
            .build()
            .unwrap();
    }
    bterm.with_post_scanlines(true);

    bevy_app::App::build()
        .add_plugin(CorePlugin::default())
        .add_stage_after(
            CoreStage::Update,
            RENDER_STAGE,
            SystemStage::single_threaded(),
        )
        .add_plugin(BracketLibPlugin::new(bterm))
        // Initialization logic
        .add_startup_system(init.system())
        // Handle input first. Input is what triggers the game to update.
        .add_system(systems::input::player_input_system.system().label("input"))
        // Run indexing systems after input to ensure that state is in sync.
        .add_system_set(
            SystemSet::new()
                .after("input")
                .label("indexing")
                .with_run_criteria(run_if_input_or_initial.system())
                .with_system(systems::visibility::visibility_system.system())
                .with_system(systems::map_indexing::map_indexing_system.system()),
        )
        // Run update systems
        .add_system(
            systems::monster_ai::monster_ai_system
                .system()
                .after("indexing")
                .with_run_criteria(run_if_input.system()),
        )
        // Rendering runs on the render stage after everything else.
        .add_system_set_to_stage(
            RENDER_STAGE,
            SystemSet::new().with_system(
                render::render
                    .system()
                    .chain(gui::render_ui_system.system()),
            ),
        )
        .run();
}

/// Initialization for entities and resources.
fn init(mut commands: Commands) {
    use components::{
        CombatStats, EntityName, Player, PlayerBundle, Position, Renderable, Viewshed,
    };

    // Spawn entities.
    commands.spawn_bundle(PlayerBundle {
        player: Player,
        name: EntityName {
            name: "Player".to_string(),
        },
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

    // Spawn resources.

    // Tile map resource.
    let map = map::Map::new_random(100, 100, &mut commands);
    commands.insert_resource(map);
    // Game log resource.
    commands.insert_resource(gamelog::GameLog {
        entries: vec!["Welcome to Tailarc!".to_string()],
    });
    // Input state resource.
    commands.insert_resource(InputState {
        has_input_on_frame: false,
    });

    tracing::info!("Finished initialization");
}
