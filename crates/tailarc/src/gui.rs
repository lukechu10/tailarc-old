use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{CombatStats, Player};
use crate::gamelog::GameLog;
use crate::map::Map;
use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

/// Render in game UI.
pub fn render_ui_system(
    mut ctx: ResMut<BTerm>,
    map: Res<Map>,
    game_log: Res<GameLog>,
    player: Query<&CombatStats, With<Player>>,
) {
    // Draw ui box.
    ctx.draw_box_double(
        0,
        CONSOLE_HEIGHT - 7,
        CONSOLE_WIDTH - 1,
        6,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );

    // Draw depth of current level.
    let depth = format!(" Depth: {} ", map.depth);
    ctx.print_color(
        2,
        CONSOLE_HEIGHT - 7,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        &depth,
    );

    // Draw player health.
    let stats = player.single().unwrap();

    let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
    ctx.print_color(
        14,
        CONSOLE_HEIGHT - 7,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        &health,
    );
    ctx.draw_bar_horizontal(
        32,
        CONSOLE_HEIGHT - 7,
        30,
        stats.hp,
        stats.max_hp,
        RGB::named(RED),
        RGB::named(BLACK),
    );

    // Draw game log.
    let mut y = CONSOLE_HEIGHT - 6;
    for log in game_log.entries.lock().unwrap().iter().rev() {
        if y < CONSOLE_HEIGHT - 1 {
            ctx.print(2, y, log);
        }
        y += 1;
    }
}
