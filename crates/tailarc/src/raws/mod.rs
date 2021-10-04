//! Code for manipulating raw files.

mod item_structs;
mod manager;
mod mob_structs;

use bevy_ecs::prelude::Commands;
use include_dir::{include_dir, Dir};
use serde::Deserialize;

use crate::components::{BlocksTile, EntityName, Mob, MobBundle, Position, Viewshed};

use self::item_structs::Consumable;
pub use self::manager::RAW_MANAGER;

/// The `/static` directory.
pub static STATIC: Dir = include_dir!("../../static");

#[derive(Debug, Deserialize, Clone)]
pub struct Raws {
    pub items: Vec<item_structs::ItemRaw>,
    pub mobs: Vec<mob_structs::MobRaw>,
}

/// Loads the raws from the `/static/spawns.json` file into memory.
pub fn load_spawns() {
    let json = STATIC.get_file("spawns.json").unwrap().contents();
    let raws: Raws = serde_json::from_slice(json).expect("could not parse spawns.json");

    RAW_MANAGER.write().load(raws);
}

pub fn get_item(name: &str) -> Option<item_structs::ItemRaw> {
    let raw_manager = RAW_MANAGER.read();
    let i = raw_manager.item_index.get(name).copied();
    i.map(|i| raw_manager.raws.items[i].clone())
}

pub fn get_mob(name: &str) -> Option<mob_structs::MobRaw> {
    let raw_manager = RAW_MANAGER.read();
    let i = raw_manager.mob_index.get(name).copied();
    i.map(|i| raw_manager.raws.mobs[i].clone())
}

#[derive(Debug, Clone, Copy)]
pub enum SpawnType {
    AtPosition(Position),
}

/// Spawns a new item.
///
/// Returns `Ok` if success or `None` if an item with this name does not exist.
pub fn try_spawn_named_item(commands: &mut Commands, name: &str, pos: SpawnType) -> Option<()> {
    let item = get_item(name)?;

    let mut e = commands.spawn();

    match pos {
        SpawnType::AtPosition(pos) => {
            e.insert(pos);
        }
    }

    if let Some(renderable) = &item.renderable {
        e.insert(*renderable);
    }
    e.insert(EntityName { name: item.name });
    e.insert(crate::components::Item);

    // Consumable.
    if let Some(Consumable { effects }) = &item.consumable {
        e.insert(*effects);
    }
    // Equippable.
    if let Some(equippable) = &item.equippable {
        e.insert(*equippable);
    }
    // Stats.
    if let Some(stats) = &item.stats {
        e.insert(*stats);
    }

    Some(())
}

/// Spawns a new mob.
///
/// Returns the spawned [`MobRaw`] or `None` if an item with this name does not exist.
pub fn try_spawn_named_mob(commands: &mut Commands, name: &str, position: SpawnType) -> Option<()> {
    let mob = get_mob(name)?;

    let mut e = commands.spawn();

    #[allow(clippy::infallible_destructuring_match)]
    let position = match position {
        SpawnType::AtPosition(pos) => pos,
        /* Uncomment this once more SpawnType variants are added.
         * _ => panic!("pos must be a SpawnType::AtPosition for mob"), */
    };

    e.insert_bundle(MobBundle {
        mob: Mob,
        name: EntityName { name: mob.name },
        position,
        renderable: mob.renderable,
        viewshed: Viewshed::new(mob.vision_range),
        blocks_tile: BlocksTile,
        combat_stats: mob.stats,
    });

    Some(())
}

/// Spawns a new entity.
///
/// # Panics
/// Panics if the entity with the given name is not found.
pub fn spawn_named_entity(commands: &mut Commands, name: &str, pos: SpawnType) {
    if try_spawn_named_item(commands, name, pos).is_some()
        || try_spawn_named_mob(commands, name, pos).is_some()
    {
        // Success!
    } else {
        panic!("could not spawn entity with name {}", name);
    }
}
