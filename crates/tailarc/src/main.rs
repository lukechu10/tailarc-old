//! **Tailarc** is a roguelike game written in Rust!

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod components;
pub mod deserialize;
pub mod gamelog;
pub mod gui;
pub mod map;
pub mod map_builders;
pub mod raws;
pub mod render;
pub mod systems;

use std::collections::HashSet;
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

/// Title of the console window.
pub const CONSOLE_TITLE: &str = "Tailarc";

/// If `true`, the entire map will be rendered regardless of whether it is explored. Useful for
/// debugging.
///
/// `true` if built in debug mode with environment variable `DEBUG_MAP_XRAY=1`. Note that the
/// environment variable is evaluated at compile time, so this is not a runtime check.
///
/// TODO: Due to const fn limitations, `DEBUG_MAP_XRAY` will be considered enabled for any arbitrary
/// value except when it is undefined.
pub const DEBUG_MAP_XRAY: bool =
    cfg!(debug_assertions) && matches!(option_env!("DEBUG_MAP_XRAY"), Some(_));

/// If `true`, the player will be given godly stats. Useful for debugging.
///
/// `true` if built in debug mode with environment variable `DEBUG_GOD_MODE=1`. Note that the
/// environment variable is evaluated at compile time, so this is not a runtime check.
///
/// TODO: Due to const fn limitations, `DEBUG_GOD_MODE` will be considered enabled for any arbitrary
/// value except when it is undefined.
pub const DEBUG_GOD_MODE: bool =
    cfg!(debug_assertions) && matches!(option_env!("DEBUG_GOD_MODE"), Some(_));

/// The current state of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunState {
    MainMenu,
    ShowInventory,
    ShowDropItem,
    AwaitingInput,
    Player,
    Monster,
}

impl RunState {
    #[track_caller]
    pub fn advance_state(state: &mut ResMut<State<Self>>) {
        let next = match state.current() {
            RunState::MainMenu => None,      // Main menu stays in main menu.
            RunState::ShowInventory => None, // Inventory does not close by itself!
            RunState::ShowDropItem => None,
            // Game loop.
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
    main_menu_result: Res<render::MainMenuResult>,
    item_menu_result: Res<gui::ItemMenuResult>,
    drop_item_result: Res<gui::DropItemResult>,
) {
    if *state.current() == RunState::MainMenu {
        if let render::MainMenuResult::Selected { selected } = *main_menu_result {
            match selected {
                render::MainMenuSelection::NewGame => state.set(RunState::AwaitingInput).unwrap(),
                render::MainMenuSelection::LoadGame => todo!("load state"),
                render::MainMenuSelection::Quit => std::process::exit(0),
            }
        }
    } else if *state.current() == RunState::ShowInventory {
        match *item_menu_result {
            gui::ItemMenuResult::Cancel => state.set(RunState::AwaitingInput).unwrap(),
            gui::ItemMenuResult::NoResponse => {}
            gui::ItemMenuResult::Selected => state.set(RunState::Player).unwrap(), /* Using an item takes up a turn. */
        }
    } else if *state.current() == RunState::ShowDropItem {
        match *drop_item_result {
            gui::DropItemResult::Cancel => state.set(RunState::AwaitingInput).unwrap(),
            gui::DropItemResult::NoResponse => {}
            gui::DropItemResult::Selected => state.set(RunState::Player).unwrap(), /* Using an item takes up a turn. */
        }
    } else if *state.current() != RunState::AwaitingInput {
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

/// Labels used in [`CoreStage::Update`].
#[derive(SystemLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateLabel {
    Input,
    Indexing,
}

/// Labels used in [`AppStages::CleanupAndRender`].
#[derive(SystemLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderLabel {
    Map,
    UiAndParticles,
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

/// Entrypoint. Code execution starts here.
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    #[cfg(feature = "trace")]
    tracing_subscriber::fmt::init();

    let mut bterm = BTermBuilder::simple(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .unwrap()
        .with_title(CONSOLE_TITLE)
        .build()
        .unwrap();
    bterm.with_post_scanlines(true);

    // Load the raws.
    raws::load_spawns();

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
                    .label(UpdateLabel::Input),
            ),
        )
        // Handle player actions.
        .add_system_set(
            SystemSet::on_update(RunState::Player)
                .with_system(systems::use_item::use_item_system.system())
                .with_system(systems::drop_item::drop_item_system.system()),
        )
        // Run indexing systems after input to ensure that state is in sync.
        // These don't need to be in a separate stage from CoreStage::Update because input doesn't
        // spawn new entities/components.
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_in_game.system())
                .label(UpdateLabel::Indexing)
                .after(UpdateLabel::Input)
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
                .with_system(render::render_game_system.system().label(RenderLabel::Map))
                .with_system(
                    gui::render_ui_system
                        .system()
                        .label(RenderLabel::UiAndParticles)
                        .after(RenderLabel::Map),
                )
                // We can run these systems in parallel with rendering because they perform cleanup
                // code for the tick. Commands are queued until next stage so render will
                // still be consistent.
                .with_system(systems::inventory::item_collection_system.system())
                .with_system(systems::damage::delete_the_dead.system())
                .with_system(systems::particle::spawn_particles_system.system())
                .with_system(systems::particle::cull_particles_system.system()),
        )
        .add_system_set_to_stage(
            AppStages::CleanupAndRender,
            SystemSet::on_update(RunState::ShowInventory).with_system(
                gui::render_inventory
                    .system()
                    .label(RenderLabel::UiAndParticles)
                    .after(RenderLabel::Map),
            ),
        )
        .add_system_set_to_stage(
            AppStages::CleanupAndRender,
            SystemSet::on_update(RunState::ShowDropItem).with_system(
                gui::render_drop_item_menu
                    .system()
                    .label(RenderLabel::UiAndParticles)
                    .after(RenderLabel::Map),
            ),
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
/// TODO: move this into a sub-module.
fn init(mut commands: Commands) {
    use components::{
        CanSufferDamage, CombatStats, EntityName, Player, PlayerBundle, Renderable, Viewshed,
    };
    use map_builders::{
        AreaStartingPosition, CullUnreachable, DrunkardsWalk, MapBuilderChain, XStart, YStart,
    };

    // Generate map.
    let mut builder = MapBuilderChain::new(80, 50, 1, DrunkardsWalk::winding_passages())
        .with(AreaStartingPosition::new(XStart::Center, YStart::Middle))
        .with(CullUnreachable);

    let map = builder.build_map();
    let starting_position = builder.starting_position();

    // Spawn monsters.
    builder.spawn_entities(&mut commands);

    // Spawn player.
    let combat_stats = if DEBUG_GOD_MODE {
        CombatStats {
            hp: 1000,
            max_hp: 1000,
            defense: 1000,
            power: 1000,
        }
    } else {
        CombatStats {
            hp: 100,
            max_hp: 100,
            defense: 2,
            power: 5,
        }
    };
    commands.spawn_bundle(PlayerBundle {
        player: Player,
        name: EntityName {
            name: "Player".to_string(),
        },
        position: starting_position,
        renderable: Renderable {
            glyph: '@' as u16,
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
            z_index: 3,
        },
        viewshed: Viewshed {
            visible_tiles: HashSet::new(),
            range: 8,
            dirty: true,
        },
        combat_stats,
        can_suffer_damage: CanSufferDamage::default(),
    });

    // Spawn resources.

    // Tile map resource.
    commands.insert_resource(map);
    // Game log resource.
    commands.insert_resource(gamelog::GameLog {
        entries: Mutex::new(vec!["Welcome to Tailarc!".to_string()]),
    });
    commands.insert_resource(render::MainMenuResult::NoSelection {
        selected: render::MainMenuSelection::NewGame,
    });
    commands.insert_resource(gui::ItemMenuResult::NoResponse);
    commands.insert_resource(gui::DropItemResult::NoResponse);
    commands.insert_resource(systems::particle::ParticleBuilder::new());

    tracing::info!("Finished initialization");
}
