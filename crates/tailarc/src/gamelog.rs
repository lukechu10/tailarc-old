use std::sync::Mutex;

pub struct GameLog {
    pub entries: Mutex<Vec<String>>,
}

impl GameLog {
    pub fn add_entry(&self, entry: impl AsRef<str>) {
        let mut entries = self.entries.lock().unwrap();
        entries.push(String::from(entry.as_ref()));
    }
}
