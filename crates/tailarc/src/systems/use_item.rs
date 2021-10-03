use bevy_ecs::prelude::*;

use crate::components::{
    CombatStats, ConsumableEffects, EntityName, Equipment, EquipmentSlot, Equippable, Item, Owned,
    Player, WantsToUseItem,
};
use crate::gamelog::GameLog;

/// Processes all the [`WantsToUseItem`] components and removes them from the entities.
pub fn use_item_system(
    mut commands: Commands,
    game_log: Res<GameLog>,
    mut wants_use: Query<(
        Entity,
        &WantsToUseItem,
        Option<&mut CombatStats>,
        Option<&mut Equipment>,
    )>,
    owned: Query<&Owned>,
    consumables: Query<(Entity, &ConsumableEffects, &EntityName), With<Item>>,
    equippable: Query<(Entity, &Equippable, &EntityName), With<Item>>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.single().unwrap();

    for (entity, wants_use, stats, equipment) in wants_use.iter_mut() {
        let owned = owned
            .get(wants_use.item)
            .expect("cannot use an item that is not owned");

        // Make sure that owned matches with the entity that wants to use it.
        if owned.owner != entity {
            panic!("cannot use an item that you do not own");
        }

        // Consumable - apply the effect of the item
        if let Ok((item, effect, name)) = consumables.get(wants_use.item) {
            if let Some(mut stats) = stats {
                if entity == player_entity {
                    // If it is the player that is using the item, display message in game log.
                    game_log.add_entry(format!("You use {}", name.name));
                }

                if let Some(heal) = effect.heal {
                    stats.hp = i32::min(stats.max_hp, stats.hp + heal);
                }
            } else {
                // Using an item without a CombatStats component does nothing.
            }
            // Despawn the item since it has been used.
            commands.entity(item).despawn();
        } else if let Ok((item, equippable, name)) = equippable.get(wants_use.item) {
            if let Some(mut equipment) = equipment {
                match equippable.slot {
                    EquipmentSlot::Melee => equipment.melee = Some(item),
                    EquipmentSlot::Shield => equipment.shield = Some(item),
                }

                if entity == player_entity {
                    // If it is the player that is using the item, display message in game log.
                    game_log.add_entry(format!("You equip {}", name.name));
                }
            } else {
                // Using an item without an Equipment component does nothing.
            }
        } else {
            panic!("cannot use an item that is not consumable or equippable");
        }

        // Remove WantsToUseItem component from entity to prevent using the item twice.
        commands.entity(entity).remove::<WantsToUseItem>();
    }
}
