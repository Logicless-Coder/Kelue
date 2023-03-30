use std::collections::HashMap;

use super::types::{Key, Value};
use super::errors;

#[derive(Debug)]
pub struct Store {
    data: HashMap<Key, Value>
}

impl Store {
    pub fn new() -> Store {
        Store { data: HashMap::new() }
    }

    pub fn get(&self, key: &Key) -> Result<&Value, errors::KelueError> {
        match self.data.get(key) {
            Some(value) => Ok(value), 
            None => Err(errors::KelueError::KeyNotFoundError)
        }
    }

    pub fn set(&mut self, key: Key, value: Value) {
        self.data.insert(key, value); 
    }

    pub fn delete(&mut self, key: &Key) {
        self.data.remove(key);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn get_keys(&self) -> Vec<&Key> {
        self.data.keys().collect::<Vec<&Key>>()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
