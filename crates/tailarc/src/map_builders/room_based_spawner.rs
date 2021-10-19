use super::spawner::spawn_room;
use super::{MapBuilder, MetaMapBuilder};

pub struct RoomBasedSpawner;

impl MetaMapBuilder for RoomBasedSpawner {
    fn build_map(&mut self, build_data: &mut MapBuilder) {
        let rooms = build_data
            .rooms
            .as_ref()
            .expect("rooms required for RoomBasedSpawner");

        for room in rooms.iter().skip(1) {
            spawn_room(&mut build_data.spawn_list, room, 3, 4);
        }
    }
}
