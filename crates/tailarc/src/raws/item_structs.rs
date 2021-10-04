use serde::Deserialize;

use crate::components::{ConsumableEffects, Equippable, ItemStats, Renderable};

#[derive(Debug, Deserialize, Clone)]
pub struct ItemRaw {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
    pub equippable: Option<Equippable>,
    pub stats: Option<ItemStats>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Consumable {
    pub effects: ConsumableEffects,
}
