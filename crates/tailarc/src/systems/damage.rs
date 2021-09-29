use bevy_ecs::prelude::*;

use crate::components::{CombatStats, SufferDamage};

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

pub fn delete_the_dead(mut commands: Commands, q: Query<(Entity, &CombatStats)>) {
    for (entity, stats) in q.iter() {
        if stats.hp <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
