//! Code for level generation.

mod bsp_dungeon;
mod cellular_automata;
mod common;
mod cull_unreachable;
mod room_based_spawner;
mod room_based_starting_position;
mod simple_map;
mod spawner;

use bevy_ecs::prelude::Commands;

use crate::components::Position;
use crate::map::Map;
use crate::raws::{spawn_named_entity, SpawnType};

use self::common::*;

pub use self::bsp_dungeon::BspDungeon;
pub use self::cellular_automata::CellularAutomata;
pub use self::cull_unreachable::CullUnreachable;
pub use self::room_based_spawner::RoomBasedSpawner;
pub use self::room_based_starting_position::RoomBasedStartingPosition;
pub use self::simple_map::SimpleMap;

/// Contains the data used by map builders.
pub struct MapBuilder {
    pub map: Map,
    pub starting_position: Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub spawn_list: Vec<(Position, String)>,
}

/// A chain of map builders.
/// It is composed of an [`InitialMapBuilder`] and a list of [`MetaMapBuilder`]s.
pub struct MapBuilderChain {
    pub starter: Box<dyn InitialMapBuilder>,
    pub builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: MapBuilder,
}

impl MapBuilderChain {
    /// Create a new [`MapBuilderChain`] with the specified dimensions, depth, and [`InitialMapBuilder`].
    pub fn new(
        width: u32,
        height: u32,
        depth: i32,
        starter: impl InitialMapBuilder + 'static,
    ) -> Self {
        Self {
            starter: Box::new(starter),
            builders: Vec::new(),
            build_data: MapBuilder {
                map: Map::new(width, height, depth),
                starting_position: None,
                rooms: None,
                corridors: None,
                spawn_list: Vec::new(),
            },
        }
    }

    pub fn with(mut self, builder: impl MetaMapBuilder + 'static) -> Self {
        self.builders.push(Box::new(builder));
        self
    }

    pub fn build_map(&mut self) -> Map {
        self.starter.build_map(&mut self.build_data);

        for meta_builder in &mut self.builders {
            meta_builder.build_map(&mut self.build_data);
        }

        self.build_data.map.clone()
    }

    /// Gets the starting position.
    ///
    /// # Panic
    /// Panics if the starting position has not been set.
    pub fn starting_position(&self) -> Position {
        self.build_data
            .starting_position
            .expect("starting position not set")
    }

    pub fn spawn_entities(&mut self, commands: &mut Commands) {
        for entity in &self.build_data.spawn_list {
            spawn_named_entity(commands, &entity.1, SpawnType::AtPosition(entity.0));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data: &mut MapBuilder);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data: &mut MapBuilder);
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

    pub fn width(&self) -> i32 {
        i32::abs(self.x1 - self.x2)
    }

    pub fn height(&self) -> i32 {
        i32::abs(self.y1 - self.y2)
    }
}
