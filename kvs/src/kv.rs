use crate::Result;
use std::path::Path;
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

    /// Creates a new `KvStore` using the index file at the given path
    ///
    /// # Arguments
    ///
    /// `path` - the path to the index file
    ///
    /// # Example
    ///
    /// ```
    /// let mut kvs = kvs::KvStore::open("kv.indx")
    /// ```
    pub fn open(path: &Path) -> Result<KvStore> {
        unimplemented!()
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
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
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
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
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
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }
}
