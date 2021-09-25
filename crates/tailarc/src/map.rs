use std::path::Path;

use anyhow::{Context, Result};
use bracket_lib::prelude::{Algorithm2D, BaseMap, Point};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Tile {
    Wall,
    Floor,
    BrickPath,
    Grass,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Map {
    pub tiles: Vec<Tile>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub show_non_visible: bool,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32, show_non_visible: bool) -> Self {
        let tile_map_size = (width * height) as usize;

        let mut map = vec![Tile::Floor; tile_map_size];

        // Make the boundaries walls.
        for x in 0..width {
            map[Self::xy_idx_with_width(x, 0, width)] = Tile::Wall;
            map[Self::xy_idx_with_width(x, height - 1, width)] = Tile::Wall;
        }
        for y in 0..height {
            map[Self::xy_idx_with_width(0, y, width)] = Tile::Wall;
            map[Self::xy_idx_with_width(width - 1, y, width)] = Tile::Wall;
        }

        // Place some random walls.
        let mut rng = rand::thread_rng();
        for _i in 0..150 {
            let x = rng.gen_range(1..width - 1);
            let y = rng.gen_range(1..height - 1);
            let idx = Self::xy_idx_with_width(x, y, width);
            map[idx] = Tile::Wall;
        }

        Self {
            tiles: map,
            revealed_tiles: vec![false; tile_map_size],
            visible_tiles: vec![false; tile_map_size],
            show_non_visible,
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

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        matches!(self.tiles[idx], Tile::Wall)
    }
}
