mod render;
mod tile;

use std::time::Instant;

use bevy_app::App;
use bevy_core::{CorePlugin, Time};
use bevy_doryen::doryen::AppOptions;
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Input, RenderSystemExtensions, RootConsole};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res, ResMut};
use tile::{xy_idx, TileMap};
use tracing::info;

use crate::tile::TileType;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const INPUT_DELAY_SECS: f64 = 0.15;

#[derive(Default, Copy, Clone, PartialEq)]
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

    App::build()
        .add_plugin(CorePlugin::default())
        .insert_resource(DoryenPluginSettings {
            app_options: AppOptions {
                console_width: CONSOLE_WIDTH,
                console_height: CONSOLE_HEIGHT,
                screen_width: CONSOLE_WIDTH * 8,
                screen_height: CONSOLE_HEIGHT * 8,
                window_title: String::from("Tailarc"),
                font_path: String::from("terminal_8x8.png"),
                vsync: true,
                fullscreen: false,
                show_cursor: true,
                resizable: true,
                intercept_close_request: false,
            },
            ..Default::default()
        })
        .add_plugin(DoryenPlugin)
        .add_startup_system(init.system())
        .add_system(player_input.system().chain(update_player_position.system()))
        .add_system(mouse_input.system())
        .add_doryen_render_system(render::render.system())
        .run();
}

fn init(mut root_console: ResMut<RootConsole>, mut commands: Commands) {
    root_console.register_color("white", (255, 255, 255, 255));
    root_console.register_color("red", (255, 92, 92, 255));
    root_console.register_color("blue", (192, 192, 255, 255));

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        position: PlayerPosition(Position {
            x: (CONSOLE_WIDTH / 2) as i32,
            y: (CONSOLE_HEIGHT / 2) as i32,
        }),
    });

    commands.spawn_bundle(MouseBundle {
        mouse: Mouse,
        position: MousePosition(Position { x: 0, y: 0 }),
    });

    // Tile map resource.
    commands.insert_resource(tile::TileMap::new());

    commands.insert_resource(LastInputInstant(None));

    info!("Finished initialization");
}

struct LastInputInstant(Option<Instant>);

/// Get player position.
fn player_input(input: Res<Input>) -> (i32, i32) {
    let mut delta_x = 0;
    let mut delta_y = 0;
    if input.key("ArrowLeft") {
        delta_x -= 1;
    }
    if input.key("ArrowRight") {
        delta_x += 1;
    }
    if input.key("ArrowUp") {
        delta_y -= 1;
    }
    if input.key("ArrowDown") {
        delta_y += 1;
    }
    (delta_x, delta_y)
}

/// Update player position
fn update_player_position(
    In((delta_x, delta_y)): In<(i32, i32)>,
    tile_map: Res<TileMap>,
    time: Res<Time>,
    mut last_input: ResMut<LastInputInstant>,
    mut q: Query<&mut PlayerPosition, With<Player>>,
) {
    if (delta_x, delta_y) != (0, 0) {
        let now = time.last_update().unwrap_or_else(|| time.startup());
        if let Some(last_input) = last_input.0.as_mut() {
            let elapsed = now.duration_since(*last_input).as_secs_f64();
            if elapsed > INPUT_DELAY_SECS {
                // Update last input time.
                *last_input = now;
            } else {
                // Return early if two sequential inputs are too close together.
                return;
            }
        } else {
            // Initialize last input instant.
            last_input.0 = Some(now);
        }
    }
    for mut player_position in q.iter_mut() {
        let mut new_position = player_position.0;
        new_position.x = (new_position.x + delta_x)
            .max(0)
            .min(CONSOLE_WIDTH as i32 - 1);
        new_position.y = (new_position.y + delta_y)
            .max(0)
            .min(CONSOLE_HEIGHT as i32 - 1);

        if tile_map.0[xy_idx(new_position.x as u32, new_position.y as u32)] != TileType::Wall {
            player_position.0 = new_position;
        }
    }
}

/// Update mouse position.
fn mouse_input(input: Res<Input>, mut q: Query<&mut MousePosition, With<Mouse>>) {
    for mut mouse_position in q.iter_mut() {
        let new_mouse_position = input.mouse_pos();
        mouse_position.0.x = new_mouse_position.0 as i32;
        mouse_position.0.y = new_mouse_position.1 as i32;
    }
}
