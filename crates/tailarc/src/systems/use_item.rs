use bevy_ecs::prelude::*;

use crate::components::{
    CombatStats, ConsumableEffects, EntityName, Equippable, Equipped, Item, Owned, Player,
    WantsToUseItem,
};
use crate::gamelog::GameLog;

/// Processes all the [`WantsToUseItem`] components and removes them from the entities.
pub fn use_item_system(
    mut commands: Commands,
    game_log: Res<GameLog>,
    mut wants_use: Query<(Entity, &WantsToUseItem, Option<&mut CombatStats>)>,
    owned: Query<&Owned>,
    consumables: Query<(Entity, &ConsumableEffects, &EntityName), With<Item>>,
    equippables: Query<(Entity, &Equippable, &EntityName), With<Item>>,
    equipped: Query<(Entity, &Equipped, &EntityName)>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.single();

    for (entity, wants_use, stats) in wants_use.iter_mut() {
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
        } else if let Ok((item, equippable, name)) = equippables.get(wants_use.item) {
            // If another item is already equipped in the slot, remove it.
            for (equipped_entity, already_equipped, already_equipped_name) in equipped.iter() {
                if already_equipped.slot == equippable.slot {
                    commands.entity(equipped_entity).remove::<Equipped>();
                    // Add it back into the player's inventory.
                    commands
                        .entity(equipped_entity)
                        .insert(Owned { owner: entity });
                    // If it is the player that is using the item, display message in game log.
                    if entity == player_entity {
                        game_log.add_entry(format!("You unequip {}", already_equipped_name.name));
                    }
                    break;
                }
            }

            // If it is the player that is using the item, display message in game log.
            if entity == player_entity {
                game_log.add_entry(format!("You equip {}", name.name));
            }

            // Add the Equipped component to the item and remove the Owned component to prevent
            // showing up in inventory.
            commands.entity(item).insert(Equipped {
                by: entity,
                slot: equippable.slot,
            });
            commands.entity(item).remove::<Owned>();
        } else {
            game_log.add_entry("You cannot use that item");
        }

        // Remove WantsToUseItem component from entity to prevent using the item twice.
        commands.entity(entity).remove::<WantsToUseItem>();
    }
}
