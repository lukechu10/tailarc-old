#![allow(clippy::type_complexity)]

mod render;
mod tilemap;
mod visibility;

use std::collections::HashSet;
use std::path::Path;

use bevy_app::App;
use bevy_bracket_lib::BracketLibPlugin;
use bevy_core::CorePlugin;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_log::info;
use bracket_lib::prelude::*;
use tilemap::TileMap;
use visibility::{visibility_system, Viewshed};

use crate::tilemap::TileType;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Position<C> {
    x: C,
    y: C,
}

#[derive(Default)]
struct Player;

#[derive(Default, Copy, Clone, PartialEq)]
struct PlayerPosition(Position<i32>);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: PlayerPosition,
    viewshed: Viewshed,
}

#[derive(Default)]
struct Mouse;

#[derive(Default, Copy, Clone, PartialEq)]
struct MousePosition(Position<i32>);

#[derive(Bundle)]
struct MouseBundle {
    mouse: Mouse,
    position: MousePosition,
}

fn main() {
    tracing_subscriber::fmt::init();

    let font_path = Path::new("static/terminal_8x8.png");
    let font_path = font_path.canonicalize().unwrap();

    App::build()
        .add_plugin(CorePlugin::default())
        .add_plugin(BracketLibPlugin::new(
            BTermBuilder::new()
                .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, font_path.to_str().unwrap())
                .with_title("Tailarc")
                .with_font(font_path.to_str().unwrap(), 8, 8)
                .build()
                .unwrap(),
        ))
        .add_startup_system(init.system())
        .add_system(player_input.system().chain(update_player_position.system()))
        .add_system(visibility_system.system())
        .add_system(mouse_input.system())
        .add_system(render::render.system())
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle {
        player: Player,
        position: PlayerPosition(Position {
            x: (CONSOLE_WIDTH / 2) as i32,
            y: (CONSOLE_HEIGHT / 2) as i32,
        }),
        viewshed: Viewshed {
            visible_tiles: HashSet::new(),
            range: 8,
            dirty: true,
        },
    });

    commands.spawn_bundle(MouseBundle {
        mouse: Mouse,
        position: MousePosition(Position { x: 0, y: 0 }),
    });

    // Tile map resource.
    commands.insert_resource(
        tilemap::TileMap::new_from_ascii_file("static/levels/finlai.txt")
            .expect("could not create level"),
        // tilemap::TileMap::new(CONSOLE_WIDTH, CONSOLE_HEIGHT),
    );

    info!("Finished initialization");
}

/// Get player position.
fn player_input(bterm: Res<BTerm>) -> (i32, i32) {
    let mut delta_x = 0;
    let mut delta_y = 0;
    if bterm.key == Some(VirtualKeyCode::Left) {
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Right) {
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Up) {
        delta_y -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Down) {
        delta_y += 1;
    }
    (delta_x, delta_y)
}

/// Update player position
fn update_player_position(
    In((delta_x, delta_y)): In<(i32, i32)>,
    map: Res<TileMap>,
    mut q: Query<(&mut PlayerPosition, &mut Viewshed), With<Player>>,
) {
    if (delta_x, delta_y) != (0, 0) {
        for (mut player_position, mut viewshed) in q.iter_mut() {
            let mut new_position = player_position.0;
            new_position.x = (new_position.x + delta_x)
                .max(0)
                .min(map.width.saturating_sub(1) as i32);
            new_position.y = (new_position.y + delta_y)
                .max(0)
                .min(map.height.saturating_sub(1) as i32);

            if map.tiles[map.xy_idx(new_position.x as u32, new_position.y as u32)] != TileType::Wall
            {
                player_position.0 = new_position;
                viewshed.dirty = true;
            }
        }
    }
}

/// Update mouse position.
fn mouse_input(bterm: Res<BTerm>, mut q: Query<&mut MousePosition, With<Mouse>>) {
    for mut mouse_position in q.iter_mut() {
        let new_mouse_position = bterm.mouse_pos();
        mouse_position.0.x = new_mouse_position.0;
        mouse_position.0.y = new_mouse_position.1;
    }
}
