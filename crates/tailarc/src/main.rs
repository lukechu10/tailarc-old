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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunState {
    MainMenu,
    AwaitingInput,
    Player,
    Monster,
}

impl RunState {
    #[track_caller]
    pub fn advance_state(state: &mut ResMut<State<Self>>) {
        let next = match state.current() {
            RunState::MainMenu => None, // Main menu stays in main menu.
            RunState::AwaitingInput => Some(RunState::Player),
            RunState::Player => Some(RunState::Monster),
            RunState::Monster => Some(RunState::AwaitingInput),
        };
        if let Some(next) = next {
            let _ = state.set(next);
        }
    }
}

/// Advances the [`RunState`] to the next state (for the next tick).
pub fn next_turn_state_system(
    mut state: ResMut<State<RunState>>,
    main_menu_result: Res<gui::MainMenuResult>,
) {
    if *state.current() == RunState::MainMenu {
        if let gui::MainMenuResult::Selected { selected } = *main_menu_result {
            match selected {
                gui::MainMenuSelection::NewGame => state.set(RunState::AwaitingInput).unwrap(),
                gui::MainMenuSelection::LoadGame => todo!("load state"),
                gui::MainMenuSelection::Quit => std::process::exit(0),
            }
        }
    }

    if *state.current() != RunState::AwaitingInput {
        RunState::advance_state(&mut state);
    }
}

/// Run criteria for only running when in game (all states except menu states).
pub fn run_if_in_game(state: Res<State<RunState>>) -> ShouldRun {
    if *state.current() != RunState::MainMenu {
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
/// All the stages defined here run after [`CoreStage::Update`]. These stages are only used during
/// the game. Other scenes such as main menu often have all their systems run in
/// [`CoreStage::Update`].
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
    CleanupAndRender,
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
            AppStages::CleanupAndRender,
            SystemStage::parallel(),
        )
        .add_state(RunState::MainMenu)
        // Add RunState to all stages.
        .add_system_set_to_stage(AppStages::MonsterTurn, State::<RunState>::get_driver())
        .add_system_set_to_stage(AppStages::ApplyCombat, State::<RunState>::get_driver())
        .add_system_set_to_stage(AppStages::ApplyDamage, State::<RunState>::get_driver())
        .add_system_set_to_stage(AppStages::CleanupAndRender, State::<RunState>::get_driver())
        .add_plugin(BracketLibPlugin::new(bterm))
        // Initialization logic
        .add_startup_system(init.system())
        // Handle input first. Input is what triggers the game to update.
        .add_system_set(
            SystemSet::on_update(RunState::AwaitingInput).with_system(
                systems::input::player_input_system
                    .system()
                    .label(Label::Input),
            ),
        )
        // Run indexing systems after input to ensure that state is in sync.
        // These don't need to be in a separate stage from CoreStage::Update because input doesn't
        // spawn new entities/components.
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_in_game.system())
                .label(Label::Indexing)
                .after(Label::Input)
                .with_system(systems::visibility::visibility_system.system())
                .with_system(systems::map_indexing::map_indexing_system.system()),
        )
        // Run monster AI systems after indexing to ensure that they are operating on consistent
        // state.
        .add_system_set_to_stage(
            AppStages::MonsterTurn,
            SystemSet::on_update(RunState::Monster)
                .with_system(systems::monster_ai::monster_ai_system.system()),
        )
        // Run combat system to attach damage to victims.
        //
        // Monsters can add combat intention components which are handled in this stage.
        .add_system_set_to_stage(
            AppStages::ApplyCombat,
            SystemSet::new()
                .with_run_criteria(run_if_in_game.system())
                .with_system(systems::melee_combat::melee_combat_system.system()),
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
        // Render the game.
        .add_system_set_to_stage(
            AppStages::CleanupAndRender,
            SystemSet::new()
                .with_run_criteria(run_if_in_game.system())
                .with_system(
                    render::render_game_system
                        .system()
                        .chain(gui::render_ui_system.system()),
                )
                // We can run these systems in parallel with rendering because they perform cleanup
                // code for the tick. Commands are queued until next stage so render will
                // still be consistent.
                .with_system(systems::damage::delete_the_dead.system()),
        )
        .add_system_set_to_stage(
            AppStages::CleanupAndRender,
            SystemSet::on_update(RunState::MainMenu)
                .with_system(render::render_main_menu_system.system()),
        )
        // Next turn always runs.
        .add_system_to_stage(AppStages::CleanupAndRender, next_turn_state_system.system())
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
    let map = map::Map::new_random(&mut commands, 100, 100, 1);
    commands.insert_resource(map);
    // Game log resource.
    commands.insert_resource(gamelog::GameLog {
        entries: Mutex::new(vec!["Welcome to Tailarc!".to_string()]),
    });
    commands.insert_resource(gui::MainMenuResult::NoSelection {
        selected: gui::MainMenuSelection::NewGame,
    });

    tracing::info!("Finished initialization");
}
