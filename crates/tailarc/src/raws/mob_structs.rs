use serde::Deserialize;

use crate::components::{CombatStats, Renderable};

#[derive(Debug, Deserialize, Clone)]
pub struct MobRaw {
    pub name: String,
    pub renderable: Renderable,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub stats: CombatStats,
}
