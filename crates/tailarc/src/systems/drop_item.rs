use bevy_ecs::prelude::*;

use crate::components::{EntityName, Item, Owned, Player, Position, WantsToDropItem};
use crate::gamelog::GameLog;

/// Processes all the [`WantsToDropItem`] components and removes them from the entities.
pub fn drop_item_system(
    mut commands: Commands,
    game_log: Res<GameLog>,
    mut wants_drop: Query<(Entity, &WantsToDropItem, &Position)>,
    items: Query<(Entity, &EntityName, &Owned), With<Item>>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.single();

    for (entity, wants_drop, pos) in wants_drop.iter_mut() {
        let (item, name, owned) = items
            .get(wants_drop.item)
            .expect("cannot drop something that is not an item");

        // Make sure that owned matches with the entity that wants to use it.
        if owned.owner != entity {
            panic!("cannot drop something that you do not own");
        }

        // Drop the item by removing Owner and adding a Position.
        commands.entity(item).remove::<Owned>();
        commands.entity(item).insert(*pos);

        // Display message if player.
        if entity == player_entity {
            game_log.add_entry(format!("You drop the {}", name.name));
        }

        // Remove WantsToDropItem component from entity to prevent dropping the item twice.
        commands.entity(entity).remove::<WantsToDropItem>();
    }
}
