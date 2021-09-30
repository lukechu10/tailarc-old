use std::cmp::{max, min};

use bevy_ecs::prelude::Commands;
use bracket_lib::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::{
    BlocksTile, CombatStats, EntityName, Monster, MonsterBundle, Position, Renderable, Viewshed,
};
use crate::map::{Map, Tile};

use super::MapBuilder;

/// Rectangle dimensions and position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    /// Create a new [`Rect`] with the specified dimensions.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    /// Returns true if the rectangle overlaps with another one.
    pub fn intersect(&self, other: &Self) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the position of the center of the rectangle.
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub struct SimpleMapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    _depth: i32,
}

impl SimpleMapBuilder {
    pub fn new(width: u32, height: u32, depth: i32) -> Self {
        Self {
            map: Map::new(width, height, depth),
            rooms: Vec::new(),
            _depth: depth,
        }
    }

    pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = map.xy_idx(x as u32, y as u32);
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    pub fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = map.xy_idx(x as u32, y as u32);
            if idx > 0 && idx < (map.width * map.height) as usize {
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    pub fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = map.xy_idx(x as u32, y as u32);
            if idx > 0 && idx < (map.width * map.height) as usize {
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors(&mut self) {
        const MAX_ROOMS: u32 = 30;
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let mut rng = thread_rng();

        for _ in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let x = rng.gen_range(1..self.map.width - w - 1) - 1;
            let y = rng.gen_range(1..self.map.height - h - 1) - 1;
            let new_room = Rect::new(x as i32, y as i32, w as i32, h as i32);
            let mut ok = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                Self::apply_room_to_map(&mut self.map, &new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rng.gen::<bool>() {
                        Self::apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        Self::apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        Self::apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        Self::apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }
    }
}

impl MapBuilder for SimpleMapBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }
    fn build_map(&mut self) {
        self.new_map_rooms_and_corridors();
    }

    fn starting_position(&self) -> Position {
        let (x, y) = self.rooms[0].center();
        Position { x, y }
    }

    fn spawn_entities(&mut self, commands: &mut Commands) {
        // Spawn a monster in each room.
        // Do not spawn a monster in the starting room.
        for (i, room) in self.rooms.iter().skip(1).enumerate() {
            let (x, y) = room.center();
            let position = Position { x, y };

            commands.spawn_bundle(MonsterBundle {
                monster: Monster,
                name: EntityName {
                    name: format!("Goblin #{}", i),
                },
                position,
                renderable: Renderable {
                    glyph: 'g' as u16,
                    fg: RGB::named(RED),
                    bg: RGB::named(BLACK),
                },
                viewshed: Viewshed::new(8),
                blocks_tile: BlocksTile,
                combat_stats: CombatStats {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                },
            });
        }
    }
}
