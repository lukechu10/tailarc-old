mod common;
pub mod simple_map;

use bevy_ecs::prelude::*;
use common::*;

use crate::components::Position;
use crate::map::Map;

pub trait MapBuilder {
    fn get_map(&self) -> Map;
    fn build_map(&mut self);
    fn starting_position(&self) -> Position;
    fn spawn_entities(&mut self, commands: &mut Commands);
}

/// Rectangle dimensions and position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    /// Create a new [`Rect`] with the specified dimensions.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    /// Returns true if the rectangle overlaps with another one.
    pub fn intersect(&self, other: &Self) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the position of the center of the rectangle.
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
