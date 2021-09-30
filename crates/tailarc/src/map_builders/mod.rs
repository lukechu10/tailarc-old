pub mod simple_map;

use bevy_ecs::prelude::*;

use crate::components::Position;
use crate::map::Map;

pub trait MapBuilder {
    fn get_map(&self) -> Map;
    fn build_map(&mut self);
    fn starting_position(&self) -> Position;
    fn spawn_entities(&mut self, commands: &mut Commands);
}
