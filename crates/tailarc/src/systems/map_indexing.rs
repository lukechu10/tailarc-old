use bevy_ecs::prelude::*;

use crate::components::{BlocksTile, Position};
use crate::map::Map;

pub fn map_indexing_system(mut map: ResMut<Map>, blocks_tile: Query<&Position, With<BlocksTile>>) {
    map.populate_blocked();
    for pos in blocks_tile.iter() {
        let idx = map.xy_idx(pos.x as u32, pos.y as u32);
        map.blocked[idx] = true;
    }
}
