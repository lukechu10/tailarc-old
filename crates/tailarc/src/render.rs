//! `doryen-rs` rendering code.

use bevy_ecs::prelude::*;
use bracket_lib::prelude::BTerm;

use crate::tile::{TileMap, TileType};
use crate::{Mouse, MousePosition, Player, PlayerPosition, CONSOLE_WIDTH};

pub(crate) fn render(
    map: Res<TileMap>,
    mut bterm: ResMut<BTerm>,
    player_query: Query<&PlayerPosition, With<Player>>,
    mouse_query: Query<&MousePosition, With<Mouse>>,
) {
    // Display tile map.
    let mut x = 0;
    let mut y = 0;
    for tile in map.0 {
        match tile {
            TileType::Wall => bterm.set(x, y, (128, 128, 128), (0, 0, 0), '#' as u16),
            TileType::Floor => bterm.set(x, y, (255, 64, 64), (32, 32, 32), '.' as u16),
        }

        // Move the coordinates
        x += 1;
        if x >= CONSOLE_WIDTH as i32 {
            x = 0;
            y += 1;
        }
    }

    // Display player.
    for player_position in player_query.iter() {
        bterm.set(
            player_position.0.x,
            player_position.0.y,
            (255, 255, 255),
            (0, 0, 0),
            '@' as u16,
        );
    }

    // Display mouse position.
    for mouse_position in mouse_query.iter() {
        bterm.set_bg(mouse_position.0.x, mouse_position.0.y, (255, 255, 255));
    }
}
