use rand::{thread_rng, Rng};

use super::{
    apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel, InitialMapBuilder,
    MapBuilder, Rect,
};

pub struct SimpleMap;

impl SimpleMap {
    pub fn new_map_rooms_and_corridors(&mut self, build_data: &mut MapBuilder) {
        const MIN_SIZE: u32 = 6;
        const MAX_SIZE: u32 = 10;

        let mut rng = thread_rng();

        let max_rooms = (build_data.map.width * build_data.map.height) / 100;
        let mut rooms = Vec::new();

        for _ in 0..max_rooms {
            let w = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let x = rng.gen_range(1..build_data.map.width - w - 2);
            let y = rng.gen_range(1..build_data.map.height - h - 2);
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(&mut build_data.map, &new_room);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms.last().unwrap().center();
                    if rng.gen::<bool>() {
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }
        build_data.rooms = Some(rooms);
    }
}

impl InitialMapBuilder for SimpleMap {
    fn build_map(&mut self, build_data: &mut MapBuilder) {
        self.new_map_rooms_and_corridors(build_data);
    }
}
