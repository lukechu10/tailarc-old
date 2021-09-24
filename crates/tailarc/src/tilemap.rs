use bracket_lib::prelude::{Algorithm2D, BaseMap, Point};
use rand::Rng;

use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TileMap {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> Self {
        let tile_map_size = (width * height) as usize;

        let mut map = vec![TileType::Floor; tile_map_size];

        // Make the boundaries walls.
        for x in 0..width {
            map[Self::xy_idx_with_width(x, 0, width)] = TileType::Wall;
            map[Self::xy_idx_with_width(x, height - 1, width)] = TileType::Wall;
        }
        for y in 0..height {
            map[Self::xy_idx_with_width(0, y, width)] = TileType::Wall;
            map[Self::xy_idx_with_width(width - 1, y, width)] = TileType::Wall;
        }

        // Place some random walls.
        let mut rng = rand::thread_rng();
        for _i in 0..400 {
            let x = rng.gen_range(1..CONSOLE_WIDTH - 1);
            let y = rng.gen_range(1..CONSOLE_HEIGHT - 1);
            let idx = Self::xy_idx_with_width(x, y, width);
            map[idx] = TileType::Wall;
        }

        Self {
            tiles: map,
            revealed_tiles: vec![false; tile_map_size],
            visible_tiles: vec![false; tile_map_size],
            width,
            height,
        }
    }

    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn xy_idx_with_width(x: u32, y: u32, width: u32) -> usize {
        (y as usize * width as usize) + x as usize
    }
}

impl Algorithm2D for TileMap {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for TileMap {
    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}
