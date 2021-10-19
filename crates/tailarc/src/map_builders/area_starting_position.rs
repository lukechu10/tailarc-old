use bracket_lib::prelude::*;

use crate::components::Position;
use crate::map::Tile;

use super::{MapBuilder, MetaMapBuilder};

pub enum XStart {
    Left,
    Center,
    Right,
}
pub enum YStart {
    Top,
    Middle,
    Bottom,
}

pub struct AreaStartingPosition {
    x: XStart,
    y: YStart,
}

impl AreaStartingPosition {
    pub fn new(x: XStart, y: YStart) -> Self {
        Self { x, y }
    }
}

impl MetaMapBuilder for AreaStartingPosition {
    fn build_map(&mut self, build_data: &mut MapBuilder) {
        let seed_x = match self.x {
            XStart::Left => 1,
            XStart::Center => build_data.map.width / 2,
            XStart::Right => build_data.map.width - 2,
        };

        let seed_y = match self.y {
            YStart::Top => 1,
            YStart::Middle => build_data.map.height / 2,
            YStart::Bottom => build_data.map.height - 2,
        };

        let mut available_floors = Vec::new();

        for idx in build_data
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, &t)| t == Tile::Floor)
            .map(|(idx, _)| idx)
        {
            let x = idx as u32 % build_data.map.width;
            let y = idx as u32 / build_data.map.width;
            let distance = DistanceAlg::PythagorasSquared
                .distance2d(Point::new(seed_x, seed_y), Point::new(x, y));
            available_floors.push((idx, distance));
        }

        // Get closest available floor.
        let closest_idx = available_floors
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        build_data.starting_position = Some(Position {
            x: closest_idx as i32 % build_data.map.width as i32,
            y: closest_idx as i32 / build_data.map.width as i32,
        });
    }
}
