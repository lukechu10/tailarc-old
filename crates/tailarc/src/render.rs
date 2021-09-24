//! `doryen-rs` rendering code.

use bevy_ecs::prelude::*;
use bracket_lib::prelude::{BTerm, RGB};

use crate::tile::{TileMap, TileType};
use crate::{PlayerPosition, CONSOLE_WIDTH};

pub(crate) fn render(
    map: Res<TileMap>,
    mut bterm: ResMut<BTerm>,
    player_query: Query<&PlayerPosition>,
) {
    let player_position = player_query.single().unwrap();
    let mut x = 0;
    let mut y = 0;

    for ((tile, revealed), visible) in map
        .tiles
        .iter()
        .zip(map.revealed_tiles.iter())
        .zip(map.visible_tiles.iter())
    {
        if *revealed {
            let glyph;
            let mut fg;
            match tile {
                TileType::Wall => {
                    fg = RGB::from_u8(128, 128, 128);
                    glyph = '#' as u16;
                }
                TileType::Floor => {
                    fg = RGB::from_u8(52, 232, 235);
                    glyph = '.' as u16;
                }
            }
            if !*visible {
                fg = fg.to_greyscale();
            }
            bterm.set(x, y, fg, RGB::from_u8(0, 0, 0), glyph);
        }
        // Move the coordinates
        x += 1;
        if x >= CONSOLE_WIDTH as i32 {
            x = 0;
            y += 1;
        }
    }

    // Display player.
    bterm.set(
        player_position.0.x,
        player_position.0.y,
        (255, 255, 255),
        (0, 0, 0),
        '@' as u16,
    );
}
