use serde::Deserialize;

use crate::components::{ConsumableEffects, Equippable, Renderable};

#[derive(Debug, Deserialize, Clone)]
pub struct ItemRaw {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
    pub equippable: Option<Equippable>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Consumable {
    pub effects: ConsumableEffects,
}
