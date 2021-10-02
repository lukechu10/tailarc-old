use rand::{thread_rng, Rng};

use crate::components::Position;
use crate::map::Tile;

use super::InitialMapBuilder;

pub struct CellularAutomata;

impl InitialMapBuilder for CellularAutomata {
    fn build_map(&mut self, build_data: &mut super::MapBuilder) {
        let mut rng = thread_rng();

        // Fill the map with 55% floor and 45% wall.
        for tile in &mut build_data.map.tiles {
            let roll = rng.gen_range(0..100);
            if roll < 45 {
                *tile = Tile::Floor;
            } else {
                *tile = Tile::Wall;
            }
        }

        // Find a starting point; start at the middle and walk left until we find an open tile.
        let mut starting_position = Position {
            x: (build_data.map.width / 2) as i32,
            y: (build_data.map.height / 2) as i32,
        };
        let mut start_idx = build_data
            .map
            .xy_idx(starting_position.x as u32, starting_position.y as u32);
        while build_data.map.tiles[start_idx] != Tile::Floor {
            starting_position.x -= 1;
            start_idx = build_data
                .map
                .xy_idx(starting_position.x as u32, starting_position.y as u32);
        }
        build_data.starting_position = Some(starting_position);
    }
}
