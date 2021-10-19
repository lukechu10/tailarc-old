//! Utilities for spawning entities in levels.

use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use crate::components::Position;
use crate::raws::RAW_MANAGER;

use super::Rect;

/// Fills a room with stuff!
pub fn spawn_room(
    spawn_list: &mut Vec<(Position, String)>,
    room: &Rect,
    max_monsters: u32,
    max_items: u32,
) {
    let mut monster_spawn_points: Vec<Position> = Vec::new();
    let mut item_spawn_points: Vec<Position> = Vec::new();

    let mut rng = thread_rng();
    let num_monsters = rng.gen_range(1..=max_monsters);
    let num_items = rng.gen_range(1..=max_items);

    for _i in 0..num_monsters {
        let mut added = false;
        while !added {
            // Generate random coordinates x and y inside room.
            let x = room.x1 + rng.gen_range(1..room.width());
            let y = room.y1 + rng.gen_range(1..room.height());
            // Ensure that the coordinates are inside the room.
            debug_assert!(room.x1 < x && x < room.x2);
            debug_assert!(room.y1 < y && y < room.y2);

            let pos = Position { x, y };
            // Check that there is not already a monster at that position.
            if !monster_spawn_points.contains(&pos) {
                monster_spawn_points.push(pos);
                added = true;
            }
        }
    }

    for _i in 0..num_items {
        let mut added = false;
        while !added {
            // Generate random coordinates x and y inside room.
            let x = room.x1 + rng.gen_range(1..room.width());
            let y = room.y1 + rng.gen_range(1..room.height());
            // Ensure that the coordinates are inside the room.
            debug_assert!(room.x1 < x && x < room.x2);
            debug_assert!(room.y1 < y && y < room.y2);

            let pos = Position { x, y };
            // Check that there is not already an item at that position.
            if !item_spawn_points.contains(&pos) {
                item_spawn_points.push(pos);
                added = true;
            }
        }
    }

    // Actually spawn the monsters.
    for &pos in monster_spawn_points.iter() {
        spawn_random_monster(spawn_list, pos);
    }

    // Actually spawn the items.
    for &pos in item_spawn_points.iter() {
        spawn_random_item(spawn_list, pos);
    }
}

/// Spawn a random monster at the specified position.
fn spawn_random_monster(spawn_list: &mut Vec<(Position, String)>, pos: Position) {
    let mut rng = thread_rng();

    let raw_manager = RAW_MANAGER.read();

    let (mob, _) = raw_manager
        .mob_index
        .iter()
        .choose(&mut rng)
        .expect("mob_index is not empty");

    spawn_list.push((pos, mob.clone()));
}

/// Spawn a random monster at the specified position.
fn spawn_random_item(spawn_list: &mut Vec<(Position, String)>, pos: Position) {
    let mut rng = thread_rng();

    let raw_manager = RAW_MANAGER.read();

    let (item, _) = raw_manager
        .item_index
        .iter()
        .choose(&mut rng)
        .expect("item_index is not empty");

    spawn_list.push((pos, item.clone()));
}
