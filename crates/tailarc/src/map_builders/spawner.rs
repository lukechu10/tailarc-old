use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use crate::components::Position;
use crate::map::Map;
use crate::raws::RAW_MANAGER;

use super::Rect;

/// Fills a room with stuff!
pub fn spawn_room(
    map: &Map,
    spawn_list: &mut Vec<(Position, String)>,
    room: &Rect,
    max_monsters: u32,
    max_items: u32,
) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spawn_points: Vec<usize> = Vec::new();

    let mut rng = thread_rng();
    let num_monsters = rng.gen_range(1..=max_monsters);
    let num_items = rng.gen_range(1..=max_items);

    for _i in 0..num_monsters {
        let mut added = false;
        while !added {
            let x = (room.x1 + rng.gen_range(1..i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.gen_range(1..i32::abs(room.y2 - room.y1))) as usize;
            let idx = (y * map.width as usize) + x;
            if !monster_spawn_points.contains(&idx) {
                monster_spawn_points.push(idx);
                added = true;
            }
        }
    }

    for _i in 0..num_items {
        let mut added = false;
        while !added {
            let x = (room.x1 + rng.gen_range(1..i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.gen_range(1..i32::abs(room.y2 - room.y1))) as usize;
            let idx = (y * map.width as usize) + x;
            if !item_spawn_points.contains(&idx) {
                item_spawn_points.push(idx);
                added = true;
            }
        }
    }

    // Actually spawn the monsters.
    for idx in monster_spawn_points.iter() {
        let x = *idx % map.width as usize;
        let y = *idx / map.height as usize;
        spawn_random_monster(
            spawn_list,
            Position {
                x: x as i32,
                y: y as i32,
            },
        );
    }

    // Actually spawn the items.
    for idx in item_spawn_points.iter() {
        let x = *idx % map.width as usize;
        let y = *idx / map.height as usize;
        spawn_random_item(
            spawn_list,
            Position {
                x: x as i32,
                y: y as i32,
            },
        );
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
