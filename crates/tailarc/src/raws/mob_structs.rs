use serde::Deserialize;

use crate::components::{CombatStats, Renderable};

#[derive(Debug, Deserialize, Clone)]
pub struct Mob {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub stats: CombatStats,
}
