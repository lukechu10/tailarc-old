use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::gamelog::GameLog;
use crate::{CombatStats, Player, CONSOLE_HEIGHT, CONSOLE_WIDTH};

pub(crate) fn render_ui_system(
    mut bterm: ResMut<BTerm>,
    game_log: Res<GameLog>,
    player: Query<&CombatStats, With<Player>>,
) {
    // Draw ui box.
    bterm.draw_box_double(
        0,
        CONSOLE_HEIGHT - 6,
        CONSOLE_WIDTH - 1,
        5,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );

    // Draw player health.
    let stats = player.single().unwrap();

    let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
    bterm.print_color(
        2,
        CONSOLE_HEIGHT - 6,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        &health,
    );
    bterm.draw_bar_horizontal(
        20,
        CONSOLE_HEIGHT - 6,
        50,
        stats.hp,
        stats.max_hp,
        RGB::named(RED),
        RGB::named(BLACK),
    );

    // Draw game log.
    let mut y = CONSOLE_HEIGHT - 5;
    for log in game_log.entries.iter().rev() {
        if y < CONSOLE_HEIGHT - 1 {
            bterm.print(2, y, log);
        }
        y += 1;
    }
}
