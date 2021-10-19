use rand::prelude::{SliceRandom, ThreadRng};
use rand::{thread_rng, Rng};

use crate::map::{Map, Tile};
use crate::map_builders::common::apply_room_to_map;

use super::{InitialMapBuilder, Rect};

/// Map generation using Binary Space Partition algorithm.
pub struct BspDungeon;

impl InitialMapBuilder for BspDungeon {
    fn build_map(&mut self, build_data: &mut super::MapBuilder) {
        let mut rng = thread_rng();

        // We want a ratio of 1 split for every 16 tiles.
        let n = build_data.map.width * build_data.map.height / 16;

        let mut rooms = Vec::new();

        // Start with a map sized rectangle.
        let mut rects = vec![Rect::new(
            2,
            2,
            build_data.map.width as i32 - 5,
            build_data.map.height as i32 - 5,
        )];
        let first_room = rects[0];
        add_subrects(&mut rects, first_room);

        for _i in 0..n {
            // Rationale: a least one element because of first_room.
            let rect = *rects.choose(&mut rng).unwrap();
            let candidate = get_random_subrect(rect, &mut rng);

            if is_possible(&build_data.map, candidate) {
                apply_room_to_map(&mut build_data.map, &candidate);
                rooms.push(candidate);
                add_subrects(&mut rects, rect);
            }
        }

        // Add in corridors.

        // Sort the rooms by left coordinate.
        rooms.sort_unstable_by(|a, b| a.x1.cmp(&b.x1));

        for i in 0..rooms.len() - 1 {
            let room = rooms[i];
            let next_room = rooms[i + 1];
            let start_x = room.x1 + rng.gen_range(0..room.width());
            let start_y = room.y1 + rng.gen_range(0..room.height());
            let end_x = next_room.x1 + rng.gen_range(0..next_room.width());
            let end_y = next_room.y1 + rng.gen_range(0..next_room.height());
            draw_corridor(&mut build_data.map, start_x, start_y, end_x, end_y);
        }

        build_data.rooms = Some(rooms);
    }
}

/// Divide a rectangle into 4 quadrants.
///
/// # Params
/// * `rects`: List of rectangles to which the new rectangles will be added.
/// * `r`: Rectangle to divide into 4.
fn add_subrects(rects: &mut Vec<Rect>, r: Rect) {
    let w = r.width();
    let h = r.height();
    let half_w = i32::max(w / 2, 1);
    let half_h = i32::max(h / 2, 1);

    rects.push(Rect::new(r.x1, r.y1, half_w, half_h));
    rects.push(Rect::new(r.x1, r.y1 + half_h, half_w, half_h));
    rects.push(Rect::new(r.x1 + half_w, r.y1, half_w, half_h));
    rects.push(Rect::new(r.x1 + half_w, r.y1 + half_h, half_w, half_h));
}

/// Returns a [`Rect`] with random dimensions that fits inside `r`.
/// The dimensions are at least 4x4 and at most 10x10.
fn get_random_subrect(mut r: Rect, rng: &mut ThreadRng) -> Rect {
    let outer_w = r.width();
    let outer_h = r.height();

    let w = i32::max(4, rng.gen_range(0..i32::min(outer_w, 10))) + 1;
    let h = i32::max(4, rng.gen_range(0..i32::min(outer_h, 10))) + 1;

    // Shift the rectangle a bit.
    r.x1 += rng.gen_range(0..6);
    r.y1 += rng.gen_range(0..6);

    r.x2 = r.x1 + w;
    r.y2 = r.y1 + h;

    r
}

fn is_possible(map: &Map, r: Rect) -> bool {
    let mut expanded = r;
    // Expand the rectangle by 2 in every direction.
    expanded.x1 -= 2;
    expanded.y1 -= 2;
    expanded.x2 += 2;
    expanded.y2 += 2;

    for y in expanded.y1..expanded.y2 {
        for x in expanded.x1..expanded.x2 {
            // If out of range, can't build.
            if x > map.width as i32 - 2 || y > map.height as i32 - 2 || x < 1 || y < 1 {
                return false;
            } else {
                let idx = map.xy_idx(x as u32, y as u32);
                if map.tiles[idx] != Tile::Wall {
                    return false;
                }
            }
        }
    }

    true
}

fn draw_corridor(map: &mut Map, x1: i32, y1: i32, x2: i32, y2: i32) {
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = map.xy_idx(x as u32, y as u32);
        map.tiles[idx] = Tile::Floor;
    }
}
