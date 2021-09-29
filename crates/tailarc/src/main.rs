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
use std::sync::Mutex;

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

pub const CONSOLE_TITLE: &str = "Tailarc";

/// A state that contains the current turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    Player,
    Monster,
}

impl TurnState {
    pub fn advance_state(&mut self) {
        *self = match *self {
            TurnState::AwaitingInput => TurnState::Player,
            // with input.
            TurnState::Player => TurnState::Monster,
            TurnState::Monster => TurnState::AwaitingInput,
        }
    }
}

pub fn next_turn_state_system(mut turn_state: ResMut<TurnState>) {
    if *turn_state != TurnState::AwaitingInput {
        turn_state.advance_state();
    }
}

fn run_if_awaiting_input(ts: Res<TurnState>) -> ShouldRun {
    if *ts == TurnState::AwaitingInput {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn run_if_monster_turn(ts: Res<TurnState>) -> ShouldRun {
    if *ts == TurnState::Monster {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(SystemLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Label {
    Input,
    Indexing,
}

/// All the world's a stage. And all the men and women merely players.
///
/// All the stages defined here run after [`CoreStage::Update`].
#[derive(StageLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppStages {
    /// Run monster AI.
    MonsterTurn,
    /// Add damage components to the victims.
    ApplyCombat,
    /// Resolve damage on the victims.
    ApplyDamage,
    /// Last stage to execute.
    ///
    /// Entities that are dead are despawned.
    /// Game is rendered here so that it has access to latest state.
    Cleanup,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    #[cfg(feature = "trace")]
    tracing_subscriber::fmt::init();

    let mut bterm;
    if cfg!(not(target_arch = "wasm32")) {
        let font_path = Path::new("static/terminal_8x8.png");
        let font_path = font_path.canonicalize().unwrap();

        bterm = BTermBuilder::new()
            .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, font_path.to_str().unwrap())
            .with_title(CONSOLE_TITLE)
            .with_font(font_path.to_str().unwrap(), 8, 8)
            .build()
            .unwrap();
    } else {
        bterm = BTermBuilder::simple(CONSOLE_WIDTH, CONSOLE_HEIGHT)
            .unwrap()
            .with_title(CONSOLE_TITLE)
            .build()
            .unwrap();
    }
    bterm.with_post_scanlines(true);

    bevy_app::App::build()
        .add_plugin(CorePlugin::default())
        .add_stage_after(
            CoreStage::Update,
            AppStages::MonsterTurn,
            SystemStage::parallel(),
        )
        .add_stage_after(
            AppStages::MonsterTurn,
            AppStages::ApplyCombat,
            SystemStage::parallel(),
        )
        .add_stage_after(
            AppStages::ApplyCombat,
            AppStages::ApplyDamage,
            SystemStage::parallel(),
        )
        .add_stage_after(
            AppStages::ApplyDamage,
            AppStages::Cleanup,
            SystemStage::parallel(),
        )
        .add_plugin(BracketLibPlugin::new(bterm))
        .insert_resource(TurnState::AwaitingInput)
        // Initialization logic
        .add_startup_system(init.system())
        // Handle input first. Input is what triggers the game to update.
        .add_system(
            systems::input::player_input_system
                .system()
                .label(Label::Input)
                .with_run_criteria(run_if_awaiting_input.system()),
        )
        // Run indexing systems after input to ensure that state is in sync.
        // These don't need to be in a separate stage from CoreStage::Update because input doesn't
        // spawn new entities/components.
        .add_system_set(
            SystemSet::new()
                .label(Label::Indexing)
                .after(Label::Input)
                .with_system(systems::visibility::visibility_system.system())
                .with_system(systems::map_indexing::map_indexing_system.system()),
        )
        // Run monster AI systems after indexing to ensure that they are operating on consistent
        // state.
        .add_system_set_to_stage(
            AppStages::MonsterTurn,
            SystemSet::new().with_system(
                systems::monster_ai::monster_ai_system
                    .system()
                    .with_run_criteria(run_if_monster_turn.system()), /* Only run if it's the
                                                                       * monster's turn. */
            ),
        )
        // Run combat system to attach damage to victims.
        //
        // Monsters can add combat intention components which are handled in this stage.
        .add_system_set_to_stage(
            AppStages::ApplyCombat,
            SystemSet::new().with_system(systems::melee_combat::melee_combat_system.system()),
        )
        // Run damage system to apply damage from combat.
        //
        // Previous stage adds damage components. This stage resolves the damage onto the entity's
        // stats.
        .add_system_set_to_stage(
            AppStages::ApplyDamage,
            SystemSet::new().with_system(systems::damage::damage_system.system()),
        )
        // Rendering runs on the cleanup stage after everything else.
        .add_system_set_to_stage(
            AppStages::Cleanup,
            SystemSet::new()
                .with_system(
                    render::render
                        .system()
                        .chain(gui::render_ui_system.system()),
                )
                // We can run these systems in parallel with rendering because they perform cleanup
                // code for the tick. Commands are queued until next stage so render will
                // still be consistent.
                .with_system(next_turn_state_system.system())
                .with_system(systems::damage::delete_the_dead.system()),
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
        entries: Mutex::new(vec!["Welcome to Tailarc!".to_string()]),
    });

    tracing::info!("Finished initialization");
}
