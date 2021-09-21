//! `doryen-rs` rendering code.

use bevy_doryen::RootConsole;
use bevy_ecs::prelude::*;
use doryen_rs::TextAlign;

use crate::tile::{TileMap, TileType};
use crate::{
    Entities, Mouse, MousePosition, Player, PlayerPosition, CONSOLE_HEIGHT, CONSOLE_WIDTH,
};

pub(crate) fn render(
    entities: Res<Entities>,
    tile_map: Res<TileMap>,
    mut root_console: ResMut<RootConsole>,
    player_query: Query<(&PlayerPosition, With<Player>)>,
    mouse_query: Query<(&MousePosition, With<Mouse>)>,
) {
    // Display tile map.
    let mut x = 0;
    let mut y = 0;
    for tile in tile_map.0 {
        match tile {
            TileType::Wall => root_console.cell(
                x,
                y,
                Some('#' as u16),
                Some((128, 128, 128, 255)),
                Some((0, 0, 0, 255)),
            ),
            TileType::Floor => root_console.cell(
                x,
                y,
                Some('.' as u16),
                Some((255, 64, 64, 255)),
                Some((128, 32, 32, 255)),
            ),
        }

        // Move the coordinates
        x += 1;
        if x >= CONSOLE_WIDTH as i32 {
            x = 0;
            y += 1;
        }
    }

    let player_position = player_query
        .get_component::<PlayerPosition>(entities.player)
        .unwrap();

    root_console.ascii(player_position.0.x, player_position.0.y, '@' as u16);
    root_console.fore(
        player_position.0.x,
        player_position.0.y,
        (255, 255, 255, 255),
    );
    root_console.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 1) as i32,
        "#[red]arrows#[white] : move",
        TextAlign::Center,
        None,
    );

    let mouse_position = mouse_query
        .get_component::<MousePosition>(entities.mouse)
        .unwrap();

    root_console.print_color(
        (CONSOLE_WIDTH / 2) as i32,
        (CONSOLE_HEIGHT - 3) as i32,
        &format!(
            "#[white]Mouse coordinates: #[red]{}, {}",
            mouse_position.0.x, mouse_position.0.y
        ),
        TextAlign::Center,
        None,
    );
    root_console.print_color(
        5,
        5,
        "#[blue]This blue text contains a #[red]red#[] word",
        TextAlign::Left,
        None,
    );
    root_console.back(
        mouse_position.0.x as i32,
        mouse_position.0.y as i32,
        (255, 255, 255, 255),
    );
}
