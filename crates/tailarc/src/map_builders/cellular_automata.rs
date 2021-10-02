use rand::{thread_rng, Rng};

use crate::components::Position;
use crate::map::Tile;

use super::InitialMapBuilder;

/// Number of iterations to apply cellular automata to the map.
const NUM_ITERATIONS: usize = 20;

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

        // Now we iteratively apply cellular automata rules.
        for _i in 0..NUM_ITERATIONS {
            let mut newtiles = build_data.map.tiles.clone();

            for y in 1..build_data.map.height - 1 {
                for x in 1..build_data.map.width - 1 {
                    let idx = build_data.map.xy_idx(x, y);
                    let mut neighbors = 0;
                    if build_data.map.tiles[idx - 1] == Tile::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + 1] == Tile::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - build_data.map.width as usize] == Tile::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + build_data.map.width as usize] == Tile::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - (build_data.map.width as usize - 1)] == Tile::Wall
                    {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - (build_data.map.width as usize + 1)] == Tile::Wall
                    {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + (build_data.map.width as usize - 1)] == Tile::Wall
                    {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + (build_data.map.width as usize + 1)] == Tile::Wall
                    {
                        neighbors += 1;
                    }

                    if neighbors > 4 || neighbors == 0 {
                        newtiles[idx] = Tile::Wall;
                    } else {
                        newtiles[idx] = Tile::Floor;
                    }
                }
            }

            build_data.map.tiles = newtiles;
        }
    }
}
