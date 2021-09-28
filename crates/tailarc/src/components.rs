//! ECS components.

use std::collections::HashSet;

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

/// A component that gives an entity a position.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// A component that gives an entity a name.
pub struct EntityName {
    pub name: String,
}

/// A component that makes an entity block a tile (so that other entities can't pass through it).
pub struct BlocksTile;

#[derive(Clone, PartialEq)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// A component that contains the data needed to render a tile.
pub struct Renderable {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}

/// A component that adds field of view to an entity.
pub struct Viewshed {
    pub visible_tiles: HashSet<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            dirty: true,
        }
    }
}

/// Player entity.
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: EntityName,
    pub position: Position,
    pub renderable: Renderable,
    pub viewshed: Viewshed,
    pub combat_stats: CombatStats,
}

/// Monster entity.
pub struct Monster;

#[derive(Bundle)]
pub struct MonsterBundle {
    pub monster: Monster,
    pub name: EntityName,
    pub position: Position,
    pub renderable: Renderable,
    pub viewshed: Viewshed,
    pub blocks_tile: BlocksTile,
    pub combat_stats: CombatStats,
}

/// A component that indicates that an entity wants to attack.
pub struct WantsToMelee {
    pub target: Entity,
}

pub struct SufferDamage {
    pub amount: Vec<i32>,
}
