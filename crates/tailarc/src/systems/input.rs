use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{CombatStats, Item, Mob, Player, Position, Viewshed, WantsToMelee};
use crate::gamelog::GameLog;
use crate::map::Map;
use crate::RunState;

use super::inventory::pickup_item;

/// Get and update player position from input.
///
/// If input was received, update the [`RunState`] to [`RunState::Player`].
pub fn player_input_system(
    mut commands: Commands,
    bterm: Res<BTerm>,
    map: Res<Map>,
    game_log: Res<GameLog>,
    mut state: ResMut<State<RunState>>,
    mut player: Query<(Entity, &mut Position, &mut Viewshed, &CombatStats), With<Player>>,
    enemies: Query<(Entity, &CombatStats), With<Mob>>,
    items: Query<(Entity, &Item)>,
) {
    let (player_entity, mut player_pos, mut viewshed, _combat_stats) = player.single_mut().unwrap();

    // Pickup item.
    if bterm.key == Some(VirtualKeyCode::Comma) {
        pickup_item(
            &mut commands,
            *player_pos,
            player_entity,
            &map,
            &game_log,
            items,
        );
        // Picking up items is a turn.
        RunState::advance_state(&mut state);
        return;
    }

    // Show inventory.
    if bterm.key == Some(VirtualKeyCode::I) {
        // We can unwrap() here because this system is only executed during RunState::AwaitingInput.
        state.set(RunState::ShowInventory).unwrap();
        return;
    }

    // Show drop selection.
    if bterm.key == Some(VirtualKeyCode::D) {
        // We can unwrap() here because this system is only executed during RunState::AwaitingInput.
        state.set(RunState::ShowDropItem).unwrap();
        return;
    }

    // Save game.
    if bterm.key == Some(VirtualKeyCode::Escape) {
        // We can unwrap() here because this system is only executed during RunState::AwaitingInput.
        state.set(RunState::SaveGame).unwrap();
        return;
    }

    let mut delta_x = 0i32;
    let mut delta_y = 0i32;
    if bterm.key == Some(VirtualKeyCode::Left) || bterm.key == Some(VirtualKeyCode::H) {
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Right) || bterm.key == Some(VirtualKeyCode::L) {
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Up) || bterm.key == Some(VirtualKeyCode::K) {
        delta_y -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::Down) || bterm.key == Some(VirtualKeyCode::J) {
        delta_y += 1;
    }
    if bterm.key == Some(VirtualKeyCode::Y) {
        delta_y -= 1;
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::U) {
        delta_y -= 1;
        delta_x += 1;
    }
    if bterm.key == Some(VirtualKeyCode::B) {
        delta_y += 1;
        delta_x -= 1;
    }
    if bterm.key == Some(VirtualKeyCode::N) {
        delta_y += 1;
        delta_x += 1;
    }

    // If we have a delta, input has been received.
    if (delta_x, delta_y) != (0, 0) {
        // Calculate the new position.
        let mut new_position = *player_pos;
        new_position.x = (new_position.x as i32 + delta_x)
            .max(0)
            .min(map.width.saturating_sub(1) as i32) as u32;
        new_position.y = (new_position.y as i32 + delta_y)
            .max(0)
            .min(map.height.saturating_sub(1) as i32) as u32;

        let idx = map.xy_idx(new_position.x as u32, new_position.y as u32);

        // Check if the new position is blocked.
        if !map.blocked[idx] {
            *player_pos = new_position;
            viewshed.dirty = true;
        }
        // Check if monster, if so, attack.
        else {
            for &potential_target in &map.tile_content[idx] {
                let target = enemies.get(potential_target);
                if let Ok((target, _)) = target {
                    // Attack!
                    commands
                        .entity(player_entity)
                        .insert(WantsToMelee { target });
                }
            }
        }

        RunState::advance_state(&mut state);
    }
}
