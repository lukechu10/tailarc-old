//! Tilemap.

use std::collections::HashSet;

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

/// Represents a single tile of the map and its properties.
#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    /// A vec of [`Tile`]s.
    pub tiles: Vec<Tile>,
    /// Tiles that have been seen by the player but not necessarily inside FOV.
    ///
    /// Updated in [`visibility`](crate::systems::visibility) system.
    pub revealed_tiles: Vec<bool>,
    /// Tiles that are visible to the player (inside FOV).
    ///
    /// Updated in [`visibility`](crate::systems::visibility) system.
    pub visible_tiles: Vec<bool>,
    /// An index of what is in each tile.
    pub tile_content: Vec<Vec<Entity>>,
    /// Tiles that are blocked (e.g. walls, monsters, etc...).
    ///
    /// Updated in [`map_indexing`](crate::systems::map_indexing) system.
    pub blocked: Vec<bool>,

    /// Width of the tile map.
    pub width: u32,
    /// Height of the tile map.
    pub height: u32,
    /// Depth of the current level.
    pub depth: i32,

    /// Location of bloodstains.
    pub bloodstains: HashSet<usize>,
}

impl Map {
    /// Create a new map consisting entirely of solid walls.
    pub fn new(width: u32, height: u32, depth: i32) -> Self {
        let tile_map_size = (width * height) as usize;
        Self {
            tiles: vec![Tile::Wall; tile_map_size],
            revealed_tiles: vec![false; tile_map_size],
            visible_tiles: vec![false; tile_map_size],
            tile_content: vec![Vec::new(); tile_map_size],
            blocked: vec![false; tile_map_size],
            width,
            height,
            depth,
            bloodstains: HashSet::new(),
        }
    }

    pub fn populate_blocked(&mut self) {
        for (i, &tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = tile == Tile::Wall;
        }
    }

    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn xy_idx_with_width(x: u32, y: u32, width: u32) -> usize {
        (y as usize * width as usize) + x as usize
    }

    /// Clears the `tile_content` field.
    pub fn clear_content_index(&mut self) {
        for content in &mut self.tile_content {
            content.clear();
        }
    }

    /// Returns true if the tile is within the map boundaries and is not blocked.
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width as i32 - 1 || y < 1 || y > self.height as i32 - 1 {
            return false;
        }
        let idx = self.xy_idx(x as u32, y as u32);
        !self.blocked[idx]
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

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width as i32;
        let y = idx as i32 / self.width as i32;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0));
        }
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0));
        }
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0));
        }
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}
