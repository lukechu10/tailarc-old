mod render;
mod tile;

use bevy_app::App;
use bevy_doryen::doryen::AppOptions;
use bevy_doryen::{DoryenPlugin, DoryenPluginSettings, Input, RenderSystemExtensions, RootConsole};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res, ResMut};
use tracing::info;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

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
        .add_system(input.system())
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

    info!("Finished initialization");
}

fn input(
    input: Res<Input>,
    mut player_query: Query<&mut PlayerPosition, With<Player>>,
    mut mouse_query: Query<&mut MousePosition, With<Mouse>>,
) {
    // Update player position.
    for mut player_position in player_query.iter_mut() {
        if input.key("ArrowLeft") {
            player_position.0.x = (player_position.0.x - 1).max(1);
        } else if input.key("ArrowRight") {
            player_position.0.x = (player_position.0.x + 1).min(CONSOLE_WIDTH as i32 - 2);
        }
        if input.key("ArrowUp") {
            player_position.0.y = (player_position.0.y - 1).max(1);
        } else if input.key("ArrowDown") {
            player_position.0.y = (player_position.0.y + 1).min(CONSOLE_HEIGHT as i32 - 2);
        }
    }

    // Update mouse position.
    for mut mouse_position in mouse_query.iter_mut() {
        let new_mouse_position = input.mouse_pos();
        mouse_position.0.x = new_mouse_position.0 as i32;
        mouse_position.0.y = new_mouse_position.1 as i32;
    }
}
