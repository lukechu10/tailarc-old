//! ECS components.

use std::collections::HashSet;

use bevy_core::Timer;
use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// A component that contains the data needed to render a tile.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Renderable {
    #[serde(deserialize_with = "crate::deserialize::u16_from_cp437")]
    pub glyph: u16,
    #[serde(deserialize_with = "crate::deserialize::rgb_from_hex")]
    pub fg: RGB,
    #[serde(deserialize_with = "crate::deserialize::rgb_from_hex")]
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
/// Should be attached on the attacker! Not the target.
///
/// This component will be automatically removed in the
/// [`melee_combat_system`](crate::systems::melee_combat::melee_combat_system).
pub struct WantsToMelee {
    pub target: Entity,
}

pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(
        commands: &mut Commands,
        query: &mut Query<&mut SufferDamage>,
        entity: Entity,
        amount: i32,
    ) {
        if let Ok(mut suffer_damage) = query.get_mut(entity) {
            suffer_damage.amount.push(amount);
        } else {
            commands.entity(entity).insert(SufferDamage {
                amount: vec![amount],
            });
        }
    }
}

pub struct ParticleLifetime {
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct ParticleBundle {
    pub particle_lifetime: ParticleLifetime,
    pub position: Position,
    pub renderable: Renderable,
}

/// Item entity.
pub struct Item;
