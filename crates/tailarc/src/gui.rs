use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{
    CombatStats, EntityName, Item, Owned, Player, WantsToDropItem, WantsToUseItem,
};
use crate::gamelog::GameLog;
use crate::map::Map;
use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

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
    let stats = player.single();

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

/// Render in game inventory.
#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn render_inventory(
    mut commands: Commands,
    mut ctx: ResMut<BTerm>,
    mut item_menu_result: ResMut<ItemMenuResult>,
    player: Query<Entity, With<Player>>,
    items: Query<(Entity, &EntityName, &Owned), With<Item>>,
) {
    let player_entity = player.single();

    let inventory: Vec<_> = items
        .iter()
        .filter(|(_, _, i)| i.owner == player_entity)
        .collect();
    let count = inventory.len();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "Inventory",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "ESCAPE to cancel",
    );

    for (j, (_, name, _)) in inventory.iter().enumerate() {
        ctx.set(17, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('('));
        ctx.set(
            18,
            y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            97 + j as FontCharType,
        );
        ctx.set(19, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        y += 1;
    }

    *item_menu_result = match ctx.key {
        Some(VirtualKeyCode::Escape) => ItemMenuResult::Cancel,
        Some(key) => {
            let selection = letter_to_option(key);
            if selection == -1 || selection >= count as i32 {
                ItemMenuResult::NoResponse
            } else {
                // Add a WantsToUseItem component to the player with the selected item.
                let (item, _, _) = inventory[selection as usize];
                commands
                    .entity(player_entity)
                    .insert(WantsToUseItem { item });
                ItemMenuResult::Selected
            }
        }
        _ => ItemMenuResult::NoResponse,
    };
}

/// Render drop item menu.
#[derive(PartialEq, Copy, Clone)]
pub enum DropItemResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn render_drop_item_menu(
    mut commands: Commands,
    mut ctx: ResMut<BTerm>,
    mut item_menu_result: ResMut<DropItemResult>,
    player: Query<Entity, With<Player>>,
    items: Query<(Entity, &EntityName, &Owned), With<Item>>,
) {
    let player_entity = player.single();

    let inventory: Vec<_> = items
        .iter()
        .filter(|(_, _, i)| i.owner == player_entity)
        .collect();
    let count = inventory.len();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );
    ctx.print_color(18, y - 2, RGB::named(YELLOW), RGB::named(BLACK), "Drop");
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "ESCAPE to cancel",
    );

    for (j, (_, name, _)) in inventory.iter().enumerate() {
        ctx.set(17, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('('));
        ctx.set(
            18,
            y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            97 + j as FontCharType,
        );
        ctx.set(19, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        y += 1;
    }

    *item_menu_result = match ctx.key {
        Some(VirtualKeyCode::Escape) => DropItemResult::Cancel,
        Some(key) => {
            let selection = letter_to_option(key);
            if selection == -1 || selection >= count as i32 {
                DropItemResult::NoResponse
            } else {
                // Add a WantsToUseItem component to the player with the selected item.
                let (item, _, _) = inventory[selection as usize];
                commands
                    .entity(player_entity)
                    .insert(WantsToDropItem { item });
                DropItemResult::Selected
            }
        }
        _ => DropItemResult::NoResponse,
    };
}
