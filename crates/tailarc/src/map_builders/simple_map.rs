use bevy_ecs::prelude::Commands;
use bracket_lib::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::{
    BlocksTile, CombatStats, EntityName, Monster, MonsterBundle, Position, Renderable, Viewshed,
};
use crate::map::Map;

use super::{apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel, MapBuilder, Rect};

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

    pub fn new_map_rooms_and_corridors(&mut self) {
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let max_rooms = (self.map.width * self.map.height) / 100;

        let mut rng = thread_rng();

        for _ in 0..max_rooms {
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
                apply_room_to_map(&mut self.map, &new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rng.gen::<bool>() {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
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
