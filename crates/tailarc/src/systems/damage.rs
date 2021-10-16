use bevy_ecs::prelude::*;

use crate::components::{CanSufferDamage, CombatStats, EntityName, Player, Position};
use crate::gamelog::GameLog;
use crate::map::Map;
use crate::RunState;

pub fn damage_system(
    mut map: ResMut<Map>,
    mut q: Query<(&mut CombatStats, &mut CanSufferDamage, Option<&Position>)>,
) {
    for (mut stats, mut can_suffer_damage, pos) in q.iter_mut() {
        let dmg = can_suffer_damage.amount.iter().sum::<i32>();
        stats.hp -= dmg;

        if dmg != 0 {
            // Add bloodstains.
            if let Some(pos) = pos {
                let idx = map.xy_idx(pos.x as u32, pos.y as u32);
                map.bloodstains.insert(idx);
            }
        }

        // Reset can_suffer_damage amount.
        can_suffer_damage.amount.clear();
    }
}

/// Despawns the entities that have been killed.
///
/// If the player has been killed, updates the RunState to [`RunState::MainMenu`].
pub fn delete_the_dead(
    mut commands: Commands,
    mut state: ResMut<State<RunState>>,
    game_log: Res<GameLog>,
    q: Query<(Entity, &CombatStats, Option<&EntityName>, Option<&Player>)>,
) {
    for (entity, stats, name, player) in q.iter() {
        if stats.hp <= 0 {
            if player.is_some() {
                // Player died.
                game_log.add_entry("You died! :(".to_string());
                state.overwrite_replace(RunState::MainMenu).unwrap();
            } else {
                // A monster died.
                if let Some(name) = name {
                    game_log.add_entry(format!("{} is dead", name.name));
                } else {
                    // Silent death...
                }
                commands.entity(entity).despawn();
            }
        }
    }
}
