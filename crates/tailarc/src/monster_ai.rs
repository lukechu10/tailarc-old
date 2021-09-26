use bevy_app::EventReader;
use bevy_ecs::prelude::*;

use crate::{InputEvent, Monster};

pub fn monster_ai_system(mut input: EventReader<InputEvent>, monsters: Query<&Monster>) {
    for _i in input.iter() {
        for _monster in monsters.iter() {}
    }
}
