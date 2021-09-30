//! Render game state to console.

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Player, Position, Renderable};
use crate::gui::{MainMenuResult, MainMenuSelection};
use crate::map::{Map, Tile};
use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x as u32, y as u32);
    map.tiles[idx] == Tile::Wall && map.revealed_tiles[idx]
}

/// Gets the correct wall glyph to be used at a given position on the map for smooth walls.
fn wall_glyph(map: &Map, x: i32, y: i32) -> u16 {
    if x < 1 || x > (map.width - 2) as i32 || y < 1 || y > (map.height - 2) as i32 {
        return 35;
    }
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
    }
}

/// Renders the [`Map`] to the screen.
pub fn render_game_system(
    map: Res<Map>,
    mut ctx: ResMut<BTerm>,
    renderables: Query<(&Renderable, &Position)>,
    player: Query<&Position, With<Player>>,
) {
    ctx.cls();

    // Draw tiles.
    let mut x = 0;
    let mut y = 0;

    let console_width_for_map = CONSOLE_WIDTH;
    let console_height_for_map = CONSOLE_HEIGHT - 6;

    let player_pos = player.single().unwrap();
    let player_screen_pos = (console_width_for_map / 2, console_height_for_map / 2);

    for ((tile, &revealed), &visible) in map
        .tiles
        .iter()
        .zip(map.revealed_tiles.iter())
        .zip(map.visible_tiles.iter())
    {
        if revealed {
            let glyph;
            let mut fg;
            let bg;
            match tile {
                Tile::Wall => {
                    fg = RGB::from_u8(76, 235, 59);
                    glyph = wall_glyph(&map, x, y);
                }
                Tile::Floor => {
                    fg = RGB::from_u8(179, 118, 112);
                    glyph = '.' as u16;
                }
                Tile::BrickPath => {
                    fg = RGB::from_u8(0, 0, 0);
                    glyph = '.' as u16;
                }
                Tile::Grass => {
                    fg = RGB::from_u8(66, 245, 84);
                    glyph = '.' as u16;
                }
            }
            if visible {
                // Show bloodstains.
                let idx = map.xy_idx(x as u32, y as u32);
                bg = if map.bloodstains.contains(&idx) {
                    RGB::from_u8(191, 0, 0)
                } else {
                    RGB::from_u8(0, 0, 0)
                };
            } else {
                fg = fg.to_greyscale();
                bg = RGB::from_u8(0, 0, 0); // Do not show bloodstains if out of sight.
            }

            // Calculate position of tile on screen (relative to position of player).
            let x_pos = x - player_pos.x + player_screen_pos.0 as i32;
            let y_pos = y - player_pos.y + player_screen_pos.1 as i32;
            // We don't need to check if the tile is outside the screen because it is already
            // checked by the `.set` method.
            ctx.set(x_pos, y_pos, fg, bg, glyph);
        }
        // Move the coordinates
        x += 1;
        if x >= map.width as i32 {
            x = 0;
            y += 1;
        }
    }

    // Draw box around console.
    ctx.draw_hollow_box_double(
        0,
        0,
        CONSOLE_WIDTH - 1,
        CONSOLE_HEIGHT - 1,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );

    // Draw renderables.
    for (renderable, pos) in renderables.iter() {
        let idx = map.xy_idx(pos.x as u32, pos.y as u32);
        // Only draw if the tile is visible.
        if map.visible_tiles[idx] {
            ctx.set(
                pos.x - player_pos.x + player_screen_pos.0 as i32,
                pos.y - player_pos.y + player_screen_pos.1 as i32,
                renderable.fg,
                renderable.bg,
                renderable.glyph,
            );
        }
    }
}

pub fn render_main_menu_system(
    mut main_menu_result: ResMut<MainMenuResult>,
    mut ctx: ResMut<BTerm>,
) {
    ctx.cls();

    ctx.print_color_centered(
        15,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "Welcome to Tailarc!",
    );

    let selected = match *main_menu_result {
        MainMenuResult::NoSelection { selected } | MainMenuResult::Selected { selected } => {
            selected
        }
    };

    if selected == MainMenuSelection::NewGame {
        ctx.print_color_centered(24, RGB::named(MAGENTA), RGB::named(BLACK), "Begin New Game");
    } else {
        ctx.print_color_centered(24, RGB::named(WHITE), RGB::named(BLACK), "Begin New Game");
    }

    if selected == MainMenuSelection::LoadGame {
        ctx.print_color_centered(25, RGB::named(MAGENTA), RGB::named(BLACK), "Load Game");
    } else {
        ctx.print_color_centered(25, RGB::named(WHITE), RGB::named(BLACK), "Load Game");
    }

    if selected == MainMenuSelection::Quit {
        ctx.print_color_centered(26, RGB::named(MAGENTA), RGB::named(BLACK), "Quit");
    } else {
        ctx.print_color_centered(26, RGB::named(WHITE), RGB::named(BLACK), "Quit");
    }

    *main_menu_result = match ctx.key {
        None => MainMenuResult::NoSelection { selected },
        Some(key) => match key {
            VirtualKeyCode::Escape => MainMenuResult::NoSelection {
                selected: MainMenuSelection::Quit,
            },
            VirtualKeyCode::Up => {
                let new_selection = match selected {
                    MainMenuSelection::NewGame => MainMenuSelection::Quit,
                    MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                    MainMenuSelection::Quit => MainMenuSelection::LoadGame,
                };
                MainMenuResult::NoSelection {
                    selected: new_selection,
                }
            }
            VirtualKeyCode::Down => {
                let new_selection = match selected {
                    MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
                    MainMenuSelection::LoadGame => MainMenuSelection::Quit,
                    MainMenuSelection::Quit => MainMenuSelection::NewGame,
                };
                MainMenuResult::NoSelection {
                    selected: new_selection,
                }
            }
            VirtualKeyCode::Return => MainMenuResult::Selected { selected },
            _ => MainMenuResult::NoSelection { selected },
        },
    };
}
