use crate::components::Position;

use super::{MapBuilder, MetaMapBuilder};

pub struct RoomBasedStartingPosition;

impl MetaMapBuilder for RoomBasedStartingPosition {
    fn build_map(&mut self, build_data: &mut MapBuilder) {
        let rooms = build_data
            .rooms
            .as_ref()
            .expect("rooms required for RoomBasedStartingPosition");
        let (x, y) = rooms[0].center();
        build_data.starting_position = Some(Position { x, y });
    }
}
