use bevy_ecs::prelude::*;

use crate::components::{CombatStats, EntityName, SufferDamage, WantsToMelee};
use crate::gamelog::GameLog;

pub fn melee_combat_system(
    mut commands: Commands,
    mut game_log: ResMut<GameLog>,
    wants_melee: Query<(Entity, &WantsToMelee, &EntityName, &CombatStats)>,
    target_stats: Query<(&CombatStats, &EntityName)>,
    mut suffer_damage: Query<&mut SufferDamage>,
) {
    for (entity, wants_melee, name, stats) in wants_melee.iter() {
        let target = wants_melee.target;

        if let Ok((target_stats, target_name)) = target_stats.get(target) {
            let damage = i32::max(0, stats.power - target_stats.defense);

            if damage == 0 {
                game_log.entries.push(format!(
                    "{} is unable to hurt {}",
                    name.name, target_name.name
                ));
            } else {
                game_log.entries.push(format!(
                    "{} hits {} for {} hp",
                    name.name, target_name.name, damage
                ));
                SufferDamage::new_damage(&mut commands, &mut suffer_damage, target, damage);
            }
        } else {
            game_log
                .entries
                .push(format!("{} hacked at the air!", name.name));
        }

        // Remove WantsToMelee component from entity to prevent damage from being applied twice.
        commands.entity(entity).remove::<WantsToMelee>();
    }
}
