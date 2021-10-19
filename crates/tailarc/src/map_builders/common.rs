//! Common utilities used by various map builders.

use std::cmp::{max, min};

use crate::map::{Map, Tile};

use super::Rect;

pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            // Make sure room is inside map.
            debug_assert!(x < map.width as i32);
            debug_assert!(y < map.height as i32);

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
