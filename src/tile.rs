use rand::Rng;

use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TileType {
    Wall,
    Floor,
}

pub(crate) const TILE_MAP_SIZE: usize = CONSOLE_WIDTH as usize * CONSOLE_HEIGHT as usize;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TileMap(pub [TileType; TILE_MAP_SIZE]);

impl TileMap {
    pub fn new() -> Self {
        let mut map = [TileType::Floor; TILE_MAP_SIZE];

        // Make the boundaries walls.
        for x in 0..CONSOLE_WIDTH {
            map[xy_idx(x, 0)] = TileType::Wall;
            map[xy_idx(x, CONSOLE_HEIGHT - 1)] = TileType::Wall;
        }
        for y in 0..CONSOLE_HEIGHT {
            map[xy_idx(0, y)] = TileType::Wall;
            map[xy_idx(CONSOLE_WIDTH - 1, y)] = TileType::Wall;
        }

        // Place some random walls.
        let mut rng = rand::thread_rng();
        for _i in 0..400 {
            let x = rng.gen_range(1..CONSOLE_WIDTH - 1);
            let y = rng.gen_range(1..CONSOLE_HEIGHT - 1);
            let idx = xy_idx(x, y);
            map[idx] = TileType::Wall;
        }

        Self(map)
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        match self.0[idx] {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

impl Default for TileMap {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn xy_idx(x: u32, y: u32) -> usize {
    (y as usize * CONSOLE_WIDTH as usize) + x as usize
}
