use std::collections::HashMap;

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use super::Raws;

pub static RAW_MANAGER: Lazy<RwLock<RawManager>> = Lazy::new(|| RwLock::new(RawManager::new()));

pub struct RawManager {
    pub raws: Raws,
    pub item_index: HashMap<String, usize>,
}

impl RawManager {
    pub fn new() -> Self {
        Self {
            raws: Raws { items: Vec::new() },
            item_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        self.item_index = HashMap::new();
        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
    }
}

impl Default for RawManager {
    fn default() -> Self {
        Self::new()
    }
}
