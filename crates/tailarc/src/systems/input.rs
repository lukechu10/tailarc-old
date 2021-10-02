use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{CombatStats, Mob, Player, Position, Viewshed, WantsToMelee};
use crate::map::Map;
use crate::RunState;

/// Get and update player position from input.
///
/// If input was received, update the [`RunState`] to [`RunState::Player`].
pub fn player_input_system(
    mut commands: Commands,
    bterm: Res<BTerm>,
    map: Res<Map>,
    mut state: ResMut<State<RunState>>,
    mut player: Query<(Entity, &mut Position, &mut Viewshed, &CombatStats), With<Player>>,
    enemies: Query<(Entity, &CombatStats), With<Mob>>,
) {
    let (player_entity, mut player_pos, mut viewshed, _combat_stats) = player.single_mut().unwrap();

    let mut delta_x = 0;
    let mut delta_y = 0;
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
        new_position.x = (new_position.x + delta_x)
            .max(0)
            .min(map.width.saturating_sub(1) as i32);
        new_position.y = (new_position.y + delta_y)
            .max(0)
            .min(map.height.saturating_sub(1) as i32);

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
