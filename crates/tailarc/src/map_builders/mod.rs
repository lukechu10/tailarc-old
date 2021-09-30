pub mod simple_map;

use crate::components::Position;
use crate::map::Map;

pub trait MapBuilder {
    fn build_map(new_depth: i32) -> Map;
    fn starting_position() -> Position;
}
