//! `doryen-rs` rendering code.

use bevy_ecs::prelude::*;
use bracket_lib::prelude::{BTerm, RGB};

use crate::tilemap::{TileMap, TileType};
use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH, PlayerPosition};

pub(crate) fn render(
    map: Res<TileMap>,
    mut bterm: ResMut<BTerm>,
    player_query: Query<&PlayerPosition>,
) {
    // Clear the screen.
    bterm.cls();

    // Draw tiles.
    let mut x = 0;
    let mut y = 0;

    let player_pos = player_query.single().unwrap();
    let player_screen_pos = (CONSOLE_WIDTH / 2, CONSOLE_HEIGHT / 2);

    for ((tile, revealed), visible) in map
        .tiles
        .iter()
        .zip(map.revealed_tiles.iter())
        .zip(map.visible_tiles.iter())
    {
        // Calculate position of tile on screen (relative to position of player).
        let x_pos = x - player_pos.0.x + player_screen_pos.0 as i32;
        let y_pos = y - player_pos.0.y + player_screen_pos.1 as i32;

        if *revealed {
            let glyph;
            let mut fg;
            match tile {
                TileType::Wall => {
                    fg = RGB::from_u8(100, 232, 235);
                    glyph = '#' as u16;
                }
                TileType::Floor => {
                    fg = RGB::from_u8(52, 232, 235);
                    glyph = '.' as u16;
                }
                TileType::Blank => {
                    fg = RGB::from_u8(0, 0, 0);
                    glyph = ' ' as u16;
                }
            }
            if !*visible {
                fg = fg.to_greyscale();
            }
            // We don't need to check if the tile is outside the screen because it is already
            // checked by the `.set` method.
            bterm.set(x_pos, y_pos, fg, RGB::from_u8(0, 0, 0), glyph);
        }
        // Move the coordinates
        x += 1;
        if x >= map.width as i32 {
            x = 0;
            y += 1;
        }
    }

    // Draw box around console.
    bterm.draw_hollow_box_double(
        0,
        0,
        CONSOLE_WIDTH - 1,
        CONSOLE_HEIGHT - 1,
        (100, 100, 100),
        (0, 0, 0),
    );

    // Display player. Player is always at the center of the screen.
    bterm.set(
        player_screen_pos.0,
        player_screen_pos.1,
        (255, 255, 255),
        (0, 0, 0),
        '@' as u16,
    );
}
