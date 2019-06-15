use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore{ map: HashMap::new() }
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        let result = match self.map.get(&key) {
            Some(s) => Some(s.to_string()),
            None => None
        };
        result
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
