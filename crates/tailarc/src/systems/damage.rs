use bevy_ecs::prelude::*;

use crate::components::{CombatStats, EntityName, Player, Position, SufferDamage};
use crate::gamelog::GameLog;
use crate::map::Map;
use crate::RunState;

pub fn damage_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut q: Query<(Entity, &mut CombatStats, &SufferDamage, Option<&Position>)>,
) {
    for (entity, mut stats, dmg, pos) in q.iter_mut() {
        stats.hp -= dmg.amount.iter().sum::<i32>();

        // Add bloodstains.
        if let Some(pos) = pos {
            let idx = map.xy_idx(pos.x as u32, pos.y as u32);
            map.bloodstains.insert(idx);
        }

        // Remove SufferDamage to prevent taking damage multiple times.
        commands.entity(entity).remove::<SufferDamage>();
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
