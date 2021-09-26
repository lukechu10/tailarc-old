use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;
use rand::Rng;

use crate::components::BlocksTile;
use crate::render::Renderable;
use crate::visibility::Viewshed;
use crate::{Monster, MonsterBundle, Position};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    BrickPath,
    Grass,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub show_non_visible: bool,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new_random(
        width: u32,
        height: u32,
        show_non_visible: bool,
        commands: &mut Commands,
    ) -> Self {
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

        // Place some random monsters.
        for _i in 0..50 {
            let x = rng.gen_range(1..width - 1) as i32;
            let y = rng.gen_range(1..height - 1) as i32;
            commands.spawn_bundle(MonsterBundle {
                monster: Monster,
                position: Position { x, y },
                renderable: Renderable {
                    glyph: 'g' as u16,
                    fg: RGB::named(RED),
                    bg: RGB::named(BLACK),
                },
                viewshed: Viewshed::new(8),
                blocks_tile: BlocksTile,
            });
        }

        Self {
            tiles: map,
            revealed_tiles: vec![false; tile_map_size],
            visible_tiles: vec![false; tile_map_size],
            blocked: vec![false; tile_map_size],
            show_non_visible,
            width,
            height,
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

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}
