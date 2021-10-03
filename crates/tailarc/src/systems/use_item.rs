use bevy_ecs::prelude::*;

use crate::components::{
    CombatStats, ConsumableEffects, EntityName, Item, Owned, Player, WantsToUseItem,
};
use crate::gamelog::GameLog;

/// Processes all the [`WantsToUseItem`] components and removes them from the entities.
pub fn use_item_system(
    mut commands: Commands,
    game_log: Res<GameLog>,
    mut wants_use: Query<(Entity, &WantsToUseItem, Option<&mut CombatStats>)>,
    items: Query<(Entity, &ConsumableEffects, &EntityName, &Owned), With<Item>>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.single().unwrap();

    for (entity, wants_use, stats) in wants_use.iter_mut() {
        let (item, effect, name, owned) = items
            .get(wants_use.item)
            .expect("cannot use something that is not an item");

        // Make sure that owned matches with the entity that wants to use it.
        if owned.owner != entity {
            panic!("cannot use something that you do not own");
        }

        // Apply the effect of the item
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

        // Remove WantsToUseItem component from entity to prevent using the item twice.
        commands.entity(entity).remove::<WantsToUseItem>();
        // Despawn the item.
        commands.entity(item).despawn();
    }
}
