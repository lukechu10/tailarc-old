use rand::{thread_rng, Rng};

use crate::map::Tile;

use super::InitialMapBuilder;

#[derive(PartialEq, Eq)]
pub enum DrunkardSpawnMode {
    Center,
    Random,
}

pub struct DrunkardsWalk {
    pub spawn_mode: DrunkardSpawnMode,
    pub lifetime: u32,
    pub floor_percent: f32,
}

impl InitialMapBuilder for DrunkardsWalk {
    fn build_map(&mut self, build_data: &mut super::MapBuilder) {
        let mut rng = thread_rng();

        let total_tiles = build_data.map.width * build_data.map.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;

        let mut floor_tile_count = build_data
            .map
            .tiles
            .iter()
            .filter(|&&x| x == Tile::Floor)
            .count();

        while floor_tile_count < desired_floor_tiles {
            let (mut x, mut y) = match self.spawn_mode {
                DrunkardSpawnMode::Center => (build_data.map.width / 2, build_data.map.height / 2),
                DrunkardSpawnMode::Random => (
                    rng.gen_range(1..build_data.map.width - 2),
                    rng.gen_range(1..build_data.map.height - 2),
                ),
            };
            let idx = build_data.map.xy_idx(x, y);
            build_data.map.tiles[idx] = Tile::Floor;

            // Make the drunkard walk around.
            for _i in 0..self.lifetime {
                let direction = rng.gen_range(0..4);
                match direction {
                    // Up.
                    0 => {
                        if x > 1 {
                            x -= 1
                        }
                    }
                    // Down.
                    1 => {
                        if x < build_data.map.width - 2 {
                            x += 1
                        }
                    }
                    // Left.
                    2 => {
                        if y > 1 {
                            y -= 1
                        }
                    }
                    // Right.
                    3 => {
                        if y < build_data.map.height - 2 {
                            y += 1
                        }
                    }
                    _ => unreachable!(),
                }
                let idx = build_data.map.xy_idx(x, y);
                build_data.map.tiles[idx] = Tile::Floor;
            }

            floor_tile_count = build_data
                .map
                .tiles
                .iter()
                .filter(|&&x| x == Tile::Floor)
                .count();
        }
    }
}

impl DrunkardsWalk {
    /// Open area preset.
    pub fn open_area() -> Self {
        Self {
            spawn_mode: DrunkardSpawnMode::Center,
            lifetime: 400,
            floor_percent: 0.5,
        }
    }

    /// Open halls preset.
    pub fn open_halls() -> Self {
        Self {
            spawn_mode: DrunkardSpawnMode::Random,
            lifetime: 400,
            floor_percent: 0.5,
        }
    }

    /// Winding passages preset.
    pub fn winding_passages() -> Self {
        Self {
            spawn_mode: DrunkardSpawnMode::Random,
            lifetime: 100,
            floor_percent: 0.4,
        }
    }
}
