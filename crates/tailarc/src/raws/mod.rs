//! Code for manipulating raw files.

mod manager;

use std::collections::HashMap;

use include_dir::{include_dir, Dir};
pub use manager::*;
use serde::{Deserialize, Serialize};

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
