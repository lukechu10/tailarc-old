//! `doryen-rs` rendering code.

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::map::{Map, Tile};
use crate::{PlayerPosition, CONSOLE_HEIGHT, CONSOLE_WIDTH};

/// Renders the [`TileMap`] to the screen.
pub(crate) fn render(
    map: Res<Map>,
    mut bterm: ResMut<BTerm>,
    player_query: Query<&PlayerPosition>,
) {
    // Clear the screen.
    bterm.cls();

    // Draw tiles.
    let mut x = 0;
    let mut y = 0;

    let console_width_for_map = CONSOLE_WIDTH;
    let console_height_for_map = CONSOLE_HEIGHT - 6;

    let player_pos = player_query.single().unwrap();
    let player_screen_pos = (console_width_for_map / 2, console_height_for_map / 2);

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
            let mut bg;
            match tile {
                Tile::Wall => {
                    fg = RGB::from_u8(179, 179, 179);
                    bg = RGB::from_u8(69, 69, 69);
                    glyph = '#' as u16;
                }
                Tile::Floor => {
                    fg = RGB::from_u8(179, 118, 112);
                    bg = RGB::from_u8(31, 23, 23);
                    glyph = '.' as u16;
                }
                Tile::BrickPath => {
                    fg = RGB::from_u8(0, 0, 0);
                    bg = RGB::from_u8(217, 125, 72);
                    glyph = '.' as u16;
                }
                Tile::Grass => {
                    fg = RGB::from_u8(66, 245, 84);
                    bg = RGB::from_u8(63, 224, 79);
                    glyph = '.' as u16;
                }
            }
            if !*visible {
                if map.show_non_visible {
                    fg = fg.to_greyscale();
                    bg = bg.to_greyscale();
                } else {
                    // Do not paint tile.
                    fg = RGB::named(BLACK);
                    bg = RGB::named(BLACK);
                }
            }
            // We don't need to check if the tile is outside the screen because it is already
            // checked by the `.set` method.
            bterm.set(x_pos, y_pos, fg, bg, glyph);
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
        RGB::named(WHITE),
        RGB::named(BLACK),
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
