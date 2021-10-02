use std::collections::HashMap;

use serde::Deserialize;

use crate::components::Renderable;

#[derive(Debug, Deserialize, Clone)]
pub struct ItemRaw {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub weapon: Option<Weapon>,
    pub consumable: Option<Consumable>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WeaponRange {
    Melee,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Weapon {
    pub range: WeaponRange,
    pub power_bonus: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Consumable {
    pub effects: HashMap<String, String>,
}
