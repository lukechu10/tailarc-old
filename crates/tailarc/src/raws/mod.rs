//! Code for manipulating raw files.

mod manager;

use std::collections::HashMap;

use bevy_ecs::world::EntityMut;
use bracket_lib::prelude::{to_cp437, RGB};
use include_dir::{include_dir, Dir};
pub use manager::*;
use serde::{Deserialize, Serialize};

use crate::components::{EntityName, Position};

/// The `/static` directory.
pub static STATIC: Dir = include_dir!("../../static");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Raws {
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    name: String,
    renderable: Option<Renderable>,
    weapon: Option<Weapon>,
    consumable: Option<Consumable>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Renderable {
    glyph: char,
    fg: String,
    bg: String,
    order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WeaponRange {
    Melee,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weapon {
    pub range: WeaponRange,
    pub power_bonus: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Consumable {
    pub effects: HashMap<String, String>,
}

pub fn load_spawns() {
    let json = STATIC.get_file("spawns.json").unwrap().contents();
    let raws: Raws = serde_json::from_slice(json).expect("could not parse spawns.json");

    RAW_MANAGER.write().load(raws);
}

pub fn get_item(name: &str) -> Option<Item> {
    let raw_manager = RAW_MANAGER.read();
    let i = raw_manager.item_index.get(name).copied();
    i.map(|i| raw_manager.raws.items[i].clone())
}

pub enum SpawnType {
    AtPosition(Position),
}

/// Adds components to the `entity` according to a named raw item.
///
/// # Panics
/// Panics if the item is not found.
pub fn init_named_item(e: &mut EntityMut, name: &str, spawn_type: SpawnType) {
    let item = get_item(name).expect("could not find item");

    match spawn_type {
        SpawnType::AtPosition(pos) => {
            e.insert(pos);
        }
    }

    // Renderable.
    if let Some(renderable) = &item.renderable {
        // Parse raw renderable into Renderable component.
        e.insert(crate::components::Renderable {
            glyph: to_cp437(renderable.glyph),
            fg: RGB::from_hex(&renderable.fg).expect("invalid hex color code"),
            bg: RGB::from_hex(&renderable.bg).expect("invalid hex color code"),
        });
    }

    e.insert(EntityName { name: item.name });
    e.insert(crate::components::Item);

    // Consumable.
    if let Some(_consumable) = &item.consumable {
        todo!("consumables");
    }

    // Weapon.
    if let Some(_weapon) = &item.weapon {
        todo!("weapons");
    }
}
