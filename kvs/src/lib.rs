#![deny(missing_docs)]

//! `kvs` is a simple in-memory key-value store.
//!
//! # About
//!
//! This key value store is an implementation of the Rust practical applications project for the
//! [PingCAP talent plan](https://github.com/pingcap/talent-plan).

use std::collections::HashMap;

/// Stores key-value relationships
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {

    /// Creates a new `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Retrieves the value for a given key (if that key is valid)
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value may be associated
    ///
    /// # Example
    ///
    /// ```
    /// let mut kvs = kvs::KvStore::new();
    /// kvs.get(String::from("key"));
    /// ```
    pub fn get(&mut self, key: String) -> Option<String> {
        match self.map.get(&key) {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }

    /// Sets a value for a given key. If the key is already present, it is overwrriten.
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value is associated
    /// `value` - the value to be associated
    ///
    /// # Example
    ///
    /// ```
    /// let mut kvs = kvs::KvStore::new();
    /// kvs.set(String::from("key"), String::from("value"));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Removes a key-value relationship. If the key is not present, nothing happens.
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value may be associated
    ///
    /// # Example
    ///
    /// ```
    /// let mut kvs = kvs::KvStore::new();
    /// kvs.remove(String::from("key"));
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
