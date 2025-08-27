// How do we handle the memory allocation?

use std::{collections::HashMap, time::{SystemTime, Duration}};

#[derive(Debug)]
pub struct Entry {
    pub value: String,
    pub expiration: Option<SystemTime>
}

#[derive(Debug)]
pub struct Dictionary {
    map: HashMap<String, Entry>
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: Entry) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.map.get(key) {
            Some(e) => Some(e.value.clone()),
            None => None
        }
    }

    pub fn del(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn exists(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    pub fn expire(&mut self, key: &str, lifetime: Duration) {
        match self.map.get_mut(key) {
            Some(entry) => {
                entry.expiration = Some(SystemTime::now() + lifetime)
            },
            // TODO
            None => {}
        }
    }

    //pub fn incr(&mut self, key: &str);
    //pub fn decr(&mut self, key: &str);
}
