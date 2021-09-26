#![allow(clippy::type_complexity)]

pub mod components;
pub mod gamelog;
pub mod gui;
pub mod map;
pub mod map_builders;
pub mod map_indexing;
pub mod monster_ai;
pub mod render;
pub mod visibility;

use std::collections::HashSet;
use std::path::Path;

use bevy_app::{App, EventWriter};
use bevy_bracket_lib::BracketLibPlugin;
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bracket_lib::prelude::*;
use components::*;
use map::Map;
use map_indexing::map_indexing_system;
use monster_ai::monster_ai_system;
use tracing::info;
use visibility::visibility_system;

pub const CONSOLE_WIDTH: u32 = 80;
pub const CONSOLE_HEIGHT: u32 = 60;

/// Event that is emitted when input is received.
pub struct InputEvent;

fn main() {
    tracing_subscriber::fmt::init();

    let font_path = Path::new("static/terminal_8x8.png");
    let font_path = font_path.canonicalize().unwrap();

    let mut bterm = BTermBuilder::new()
        .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, font_path.to_str().unwrap())
        .with_title("Tailarc")
        .with_font(font_path.to_str().unwrap(), 8, 8)
        .build()
        .unwrap();
    bterm.with_post_scanlines(true);

    App::build()
        .add_plugin(CorePlugin::default())
        .add_plugin(BracketLibPlugin::new(bterm))
        .add_event::<InputEvent>()
        .add_startup_system(init.system())
        .add_system(
            player_input
                .system()
                .chain(update_player_position.system())
                .label("input"),
        )
        .add_system_set(
            SystemSet::new()
                .after("input")
                .label("update state")
                .with_system(visibility_system.system())
                .with_system(map_indexing_system.system()),
        )
        .add_system(monster_ai_system.system().after("update state"))
        .add_system(
            render::render
                .system()
                .chain(gui::render_ui_system.system()),
        )
        .run();
}

fn init(mut commands: Commands) {
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
    let map = Map::new_random(100, 100, true, &mut commands);
    commands.insert_resource(map);
    // Game log resource.
    commands.insert_resource(gamelog::GameLog {
        entries: vec!["Welcome to Tailarc!".to_string()],
    });

    info!("Finished initialization");
}

/// Get player position.
fn player_input(bterm: Res<BTerm>) -> (i32, i32) {
    let mut delta_x = 0;
    let mut delta_y = 0;
    if bterm.key == Some(VirtualKeyCode::Left) || bterm.key == Some(VirtualKeyCode::H) {
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Right) || bterm.key == Some(VirtualKeyCode::L) {
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Up) || bterm.key == Some(VirtualKeyCode::K) {
        delta_y -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Down) || bterm.key == Some(VirtualKeyCode::J) {
        delta_y += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Y) {
        delta_y -= 1;
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::U) {
        delta_y -= 1;
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::B) {
        delta_y += 1;
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::N) {
        delta_y += 1;
        delta_x += 1;
    }
    (delta_x, delta_y)
}

/// Update player position
fn update_player_position(
    In((delta_x, delta_y)): In<(i32, i32)>,
    map: Res<Map>,
    mut input: EventWriter<InputEvent>,
    mut q: Query<(&mut Position, &mut Viewshed), With<Player>>,
) {
    if (delta_x, delta_y) != (0, 0) {
        for (mut player_position, mut viewshed) in q.iter_mut() {
            let mut new_position = *player_position;
            new_position.x = (new_position.x + delta_x)
                .max(0)
                .min(map.width.saturating_sub(1) as i32);
            new_position.y = (new_position.y + delta_y)
                .max(0)
                .min(map.height.saturating_sub(1) as i32);

            if !map.blocked[map.xy_idx(new_position.x as u32, new_position.y as u32)] {
                *player_position = new_position;
                viewshed.dirty = true;
                input.send(InputEvent);
            }
        }
    }
}
