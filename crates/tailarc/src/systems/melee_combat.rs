use std::time::Duration;

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{
    CombatStats, EntityName, Position, Renderable, SufferDamage, WantsToMelee,
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
    mut suffer_damage: Query<&mut SufferDamage>,
) {
    for (entity, wants_melee, name, stats) in wants_melee.iter() {
        let target = wants_melee.target;

        if let Ok((target_stats, target_name, position)) = target_stats.get(target) {
            let damage = i32::max(0, stats.power - target_stats.defense);

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
        commands.entity(entity).remove::<WantsToMelee>();
    }
}
