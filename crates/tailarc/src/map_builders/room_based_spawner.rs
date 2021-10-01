use rand::{thread_rng, Rng};

use crate::components::Position;

use super::{MapBuilder, MetaMapBuilder};

pub struct RoomBasedSpawner;

impl MetaMapBuilder for RoomBasedSpawner {
    fn build_map(&mut self, build_data: &mut MapBuilder) {
        let rooms = build_data
            .rooms
            .as_ref()
            .expect("rooms required for RoomBasedSpawner");

        let mut rng = thread_rng();

        for room in rooms.iter().skip(1) {
            let (x, y) = room.center();
            if rng.gen() {
                build_data
                    .spawn_list
                    .push((Position { x, y }, "Goblin".to_string()));
            } else {
                build_data
                    .spawn_list
                    .push((Position { x, y }, "Orc".to_string()));
            }
        }
    }
}
