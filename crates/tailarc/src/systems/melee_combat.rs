use std::time::Duration;

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{
    CombatStats, EntityName, Equipped, ItemStats, Position, Renderable, SufferDamage, WantsToMelee,
};
use crate::gamelog::GameLog;

use super::particle::ParticleBuilder;

/// Processes all the [`WantsToMelee`] components and removes them from the entities.
pub fn melee_combat_system(
    mut commands: Commands,
    game_log: Res<GameLog>,
    mut particle_builder: ResMut<ParticleBuilder>,
    wants_melee: Query<(Entity, &WantsToMelee, &EntityName, &CombatStats)>,
    target_stats: Query<(&CombatStats, &EntityName, Option<&Position>)>,
    equipped: Query<(&Equipped, &ItemStats)>,
    mut suffer_damage: Query<&mut SufferDamage>,
) {
    for (attacker, wants_melee, name, stats) in wants_melee.iter() {
        let target = wants_melee.target;

        if let Ok((target_stats, target_name, position)) = target_stats.get(target) {
            // Compute damage, taking into account equipped bonus.
            let attacker_power_bonus: i32 = equipped
                .iter()
                .filter(|(e, _)| e.by == attacker)
                .map(|(_, s)| s.power)
                .sum();
            let attacker_power = stats.power + attacker_power_bonus;
            let target_defense_bonus: i32 = equipped
                .iter()
                .filter(|(e, _)| e.by == target)
                .map(|(_, s)| s.defense)
                .sum();
            let target_defense = target_stats.defense + target_defense_bonus;
            let damage = i32::max(0, attacker_power - target_defense);

            if damage == 0 {
                game_log.add_entry(format!(
                    "{} is unable to hurt {}",
                    name.name, target_name.name
                ));
            } else {
                game_log.add_entry(format!(
                    "{} hits {} for {} hp",
                    name.name, target_name.name, damage
                ));
                SufferDamage::new_damage(&mut commands, &mut suffer_damage, target, damage);
                if let Some(&position) = position {
                    particle_builder.request(
                        position,
                        Renderable {
                            bg: RGB::named(BLACK),
                            fg: RGB::named(ORANGE),
                            glyph: to_cp437('‼'),
                            z_index: 4, // Particles should always be on the very top.
                        },
                        Duration::from_millis(200),
                    );
                }
            }
        } else {
            game_log.add_entry(format!("{} hacked at the air!", name.name));
        }

        // Remove WantsToMelee component from entity to prevent damage from being applied twice.
        commands.entity(attacker).remove::<WantsToMelee>();
    }
}
