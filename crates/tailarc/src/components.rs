//! ECS components.

use bevy_core::Timer;
use bevy_ecs::prelude::*;
use bevy_reflect::{Reflect, TypeRegistry};
use bracket_lib::prelude::*;
use serde::{Deserialize, Serialize};

/// Registers all the component types in the given registry.
pub fn register_component_types(type_registry: &mut TypeRegistry) {
    /// Calls [`AppBuilder::register_type`] for all types passed to the macro.
    macro_rules! register_types {
            ($($t:ty),* $(,)?) => {
                $(
                    type_registry.register::<$t>();
                )*
            };
        }

    register_types!(
        Position,
        EntityName,
        BlocksTile,
        CombatStats,
        Renderable,
        Viewshed,
        Player,
        Mob,
        WantsToMelee,
        CanSufferDamage,
        ParticleLifetime,
        Item,
        WantsToPickupItem,
        Owned,
        ConsumableEffects,
        WantsToUseItem,
        WantsToDropItem,
        EquipmentSlot,
        Equippable,
        Equipped,
        ItemStats,
    );
}

/// A component that gives an entity a position.
#[derive(Debug, Reflect, Component, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[reflect_value(Component, Serialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

/// A component that gives an entity a name.
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct EntityName {
    pub name: String,
}

/// A component that makes an entity block a tile (so that other entities can't pass through it).
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct BlocksTile;

#[derive(Debug, Reflect, Component, Default, Deserialize, Clone, Copy)]
#[reflect(Component)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub defense: i32,
    pub power: i32,
}

/// A component that contains the data needed to render a tile.
#[derive(Debug, Reflect, Component, Default, Serialize, Deserialize, Clone, Copy)]
#[reflect_value(Component, Serialize)]
pub struct Renderable {
    #[serde(deserialize_with = "crate::deserialize::u16_from_cp437")]
    pub glyph: u16,
    #[serde(deserialize_with = "crate::deserialize::rgb_from_hex")]
    pub fg: RGB,
    #[serde(deserialize_with = "crate::deserialize::rgb_from_hex")]
    pub bg: RGB,
    /// The order in which this tile should be rendered relative to other tiles.
    /// Higher values go on top of lower values.
    ///
    /// # Values
    /// * `0`: Default (tile)
    /// * `1`: Items
    /// * `2`: Mobs
    /// * `3`: Player
    /// * `4`: Particles
    pub z_index: i32,
}

/// A component that adds field of view to an entity.
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Viewshed {
    pub visible_tiles: bevy_utils::HashSet<Position>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: bevy_utils::HashSet::default(),
            range,
            dirty: true,
        }
    }
}

/// Player entity.
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: EntityName,
    pub position: Position,
    pub renderable: Renderable,
    pub viewshed: Viewshed,
    pub combat_stats: CombatStats,
    pub can_suffer_damage: CanSufferDamage,
}

/// Mob entity.
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Mob;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub name: EntityName,
    pub position: Position,
    pub renderable: Renderable,
    pub viewshed: Viewshed,
    pub blocks_tile: BlocksTile,
    pub combat_stats: CombatStats,
    pub can_suffer_damage: CanSufferDamage,
}

/// A component that indicates that an entity wants to attack.
/// Should be attached on the attacker! Not the target.
///
/// This component will be automatically removed in the
/// [`melee_combat_system`](crate::systems::melee_combat::melee_combat_system).
///
/// Not reflected as component.
#[derive(Debug, Reflect, Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// A component that indicates that an entity can be attacked. This component should always be
/// attached to the entity, even when it is not being attacked.
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct CanSufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
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
#[derive(Debug, Reflect, Component, Default)]
#[reflect(Component)]
pub struct Item;

/// Indicates intent to pickup an item.
// Not reflected as component.
#[derive(Debug, Reflect, Component)]
pub struct WantsToPickupItem {
    pub item: Entity,
}

/// An entity that is owned by another entity (e.g. an item that is in the player's backpack).
#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Owned {
    pub owner: Entity,
}

// TODO: this is a hack to get around the fact that `Reflect` needs `impl FromWorld`.
impl FromWorld for Owned {
    fn from_world(_world: &mut World) -> Self {
        Self {
            owner: Entity::from_raw(u32::MAX),
        }
    }
}

/// An item that has an effect when consumed.
#[derive(Debug, Reflect, Component, Default, Deserialize, Clone, Copy)]
#[reflect(Component)]
pub struct ConsumableEffects {
    pub heal: Option<i32>,
}

/// Not reflected as component.
#[derive(Debug, Reflect, Component)]
pub struct WantsToUseItem {
    /// `item` must have an [`Item`] component.
    pub item: Entity,
}

/// Not reflected as component.
#[derive(Debug, Reflect, Component)]
pub struct WantsToDropItem {
    /// `item` must have an [`Item`] component.
    pub item: Entity,
}

#[derive(Debug, Reflect, Component, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[reflect_value(Serialize)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

#[derive(Debug, Reflect, Component, Deserialize, Clone, Copy)]
#[reflect(Component)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

// TODO: this is a hack to get around the fact that `Reflect` needs `impl FromWorld`.
impl FromWorld for Equippable {
    fn from_world(_world: &mut World) -> Self {
        Self {
            slot: EquipmentSlot::Melee,
        }
    }
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Equipped {
    pub by: Entity,
    /// This field should match the `slot` field on [`Equippable`].
    pub slot: EquipmentSlot,
}

// TODO: this is a hack to get around the fact that `Reflect` needs `impl FromWorld`.
impl FromWorld for Equipped {
    fn from_world(_world: &mut World) -> Self {
        Self {
            by: Entity::from_raw(u32::MAX),
            slot: EquipmentSlot::Melee,
        }
    }
}

#[derive(Debug, Reflect, Component, Default, Deserialize, Clone, Copy)]
#[reflect(Component)]
pub struct ItemStats {
    #[serde(default)]
    pub power: i32,
    #[serde(default)]
    pub defense: i32,
}
