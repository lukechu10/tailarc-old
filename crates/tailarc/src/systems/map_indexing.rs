use bevy_ecs::prelude::*;

use crate::components::{BlocksTile, Position};
use crate::map::Map;

pub fn map_indexing_system(
    mut map: ResMut<Map>,
    blocks_tile: Query<(Entity, &Position, Option<&BlocksTile>)>,
) {
    map.populate_blocked();
    map.clear_content_index();

    for (entity, pos, blocks_tile) in blocks_tile.iter() {
        let idx = map.xy_idx(pos.x as u32, pos.y as u32);

        // If the entity blocks the tile, update the blocked tile list.
        if blocks_tile.is_some() {
            map.blocked[idx] = true;
        }

        // Set the map's tile_content at this location to be the entity's id.
        map.tile_content[idx].push(entity);
    }
}
