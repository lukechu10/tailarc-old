use std::path::Path;

use anyhow::{Context, Result};
use bracket_lib::prelude::{Algorithm2D, BaseMap, Point};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TileType {
    Wall,
    Floor,
    BrickPath,
    Grass,
}

impl TileType {
    pub fn from_ascii_byte(ch: u8) -> Self {
        match ch {
            b'#' => TileType::Wall,
            b'.' => TileType::Floor,
            b'p' => TileType::BrickPath,
            b' ' => TileType::Grass,
            _ => panic!("Unrecognized tile character {}", ch),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TileMap {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub show_non_visible: bool,
    pub width: u32,
    pub height: u32,
}

impl TileMap {
    #[allow(dead_code)] // TODO: remove
    pub fn new(width: u32, height: u32, show_non_visible: bool) -> Self {
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
        for _i in 0..150 {
            let x = rng.gen_range(1..width - 1);
            let y = rng.gen_range(1..height - 1);
            let idx = Self::xy_idx_with_width(x, y, width);
            map[idx] = TileType::Wall;
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

    pub fn new_from_ascii_file(path: impl AsRef<Path>, show_non_visible: bool) -> Result<Self> {
        let ascii = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "could not open file at {} for loading level",
                path.as_ref().display()
            )
        })?;

        // Calculate the width and height of the map.
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        for line in ascii.lines() {
            if line.len() as u32 > width {
                width = line.len() as u32;
            }
            height += 1;
        }

        let tile_map_size = (width * height) as usize;
        let mut map = vec![TileType::Floor; tile_map_size];

        // Read the map data from `ascii`.
        for (y, line) in ascii.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                let idx = Self::xy_idx_with_width(x as u32, y as u32, width);
                map[idx] = TileType::from_ascii_byte(byte);
            }
        }

        Ok(Self {
            tiles: map,
            revealed_tiles: vec![false; tile_map_size],
            visible_tiles: vec![false; tile_map_size],
            show_non_visible,
            width,
            height,
        })
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
        matches!(self.tiles[idx], TileType::Wall)
    }
}
