use bevy_ecs::prelude::*;

use crate::components::{CombatStats, Player, SufferDamage};
use crate::gamelog::GameLog;

pub fn damage_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut CombatStats, &SufferDamage)>,
) {
    for (entity, mut stats, dmg) in q.iter_mut() {
        stats.hp -= dmg.amount.iter().sum::<i32>();

        // Remove SufferDamage to prevent taking damage multiple times.
        commands.entity(entity).remove::<SufferDamage>();
    }
}

pub fn delete_the_dead(
    mut commands: Commands,
    mut game_log: ResMut<GameLog>,
    q: Query<(Entity, &CombatStats, Option<&Player>)>,
) {
    for (entity, stats, player) in q.iter() {
        if stats.hp <= 0 {
            if player.is_some() {
                game_log.entries.push("You died! :(".to_string());
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
