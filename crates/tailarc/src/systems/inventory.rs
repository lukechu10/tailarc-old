use bevy_ecs::prelude::*;

use crate::components::{EntityName, Item, Owned, Player, Position, WantsToPickupItem};
use crate::gamelog::GameLog;
use crate::map::Map;

pub fn pickup_item(
    commands: &mut Commands,
    pos: Position,
    player_entity: Entity,
    map: &Map,
    game_log: &GameLog,
    items: Query<(Entity, &Item)>,
) {
    let idx = map.xy_idx(pos.x as u32, pos.y as u32);
    for &content in &map.tile_content[idx] {
        if let Ok((item, _)) = items.get(content) {
            commands
                .entity(player_entity)
                .insert(WantsToPickupItem { item });
            return;
        }
    }

    // If we didn't find an item, display a message.
    game_log.add_entry("There is nothing to pick up here");
}

/// Iterates over all entities with [`WantsToPickupItem`] component and collects these items by
/// adding a [`InBackpack`] component to it.
///
/// If entity has [`Player`] component, it will also add a message to the game log.
pub fn item_collection_system(
    mut commands: Commands,
    game_log: Res<GameLog>,
    player: Query<Entity, With<Player>>,
    wants_pickup: Query<(Entity, &WantsToPickupItem)>,
    item_names: Query<&EntityName, With<Item>>,
) {
    let player_entity = player.single();

    // Collect items.
    for (owner, wants_pickup) in wants_pickup.iter() {
        let target_item = wants_pickup.item;
        // Remove the `Position` component from the item.
        commands.entity(target_item).remove::<Position>();
        // Add the `InBackpack` component to the item.
        commands.entity(target_item).insert(Owned { owner });
        // Remove the `WantsToPickupItem` component because we already picked the item up.
        commands.entity(owner).remove::<WantsToPickupItem>();

        // Display a message if the player picked up an item.
        if owner == player_entity {
            if let Ok(EntityName { name }) = item_names.get(target_item) {
                game_log.add_entry(format!("You pick up the {}", name));
            } else {
                game_log.add_entry("You pick something up");
            }
        }
    }
}
