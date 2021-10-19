use bracket_lib::prelude::DijkstraMap;

use crate::map::Tile;

use super::MetaMapBuilder;

/// Remove areas that are not reachable from the starting position.
pub struct CullUnreachable;

impl MetaMapBuilder for CullUnreachable {
    fn build_map(&mut self, build_data: &mut super::MapBuilder) {
        let starting_position = build_data
            .starting_position
            .expect("CullUnreachable needs a starting position");
        let start_idx = build_data
            .map
            .xy_idx(starting_position.x as u32, starting_position.y as u32);
        build_data.map.populate_blocked();

        let map_starts = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(
            build_data.map.width as usize,
            build_data.map.height as usize,
            &map_starts,
            &build_data.map,
            1000.0,
        );
        for (i, tile) in build_data.map.tiles.iter_mut().enumerate() {
            if *tile == Tile::Floor {
                let distance_to_start = dijkstra_map.map[i];
                // We can't get to this tile - so we'll make it a wall.
                if distance_to_start == f32::MAX {
                    *tile = Tile::Wall;
                }
            }
        }
    }
}
