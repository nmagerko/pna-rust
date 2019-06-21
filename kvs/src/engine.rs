use crate::Result;

/// Defines a storage interface for key-value storage
pub trait KvsEngine {
    /// Retrieves the value for a given key (if that key is valid)
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value may be associated
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Sets a value for a given key. If the key is already present, it is overwrriten.
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value is associated
    /// `value` - the value to be associated
    ///
    /// # Errors
    ///
    /// - A `KvError::IoError` will occur if file operations fail
    /// - A `KvError::SerdeError` will occur if seralizing content for the logfile fails
    fn set(&mut self, key: String, value: String) -> Result<()>;
    /// Removes a key-value relationship. If the key is not present, nothing happens.
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value may be associated
    ///
    /// # Errors
    ///
    /// - A `KvError::BadRemovalError` will occur if the requested key was not found
    /// - A `KvError::IoError` will occur if file operations fail
    /// - A `KvError::SerdeError` will occur if seralizing content for the logfile fails
    fn remove(&mut self, key: String) -> Result<()>;
}
