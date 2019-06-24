use crate::{KvsEngine, KvError, Result};
use std::{env, str};
use sled::{Db, IVec};

/// An implementation of the `sled` library that is compatible with this library's key-value store
/// interface.
pub struct SledKvsEngine {
    store: sled::Db
}

impl SledKvsEngine {
    /// Creates a new storage instance using `sled` as the storage engine
    ///
    /// # Errors
    ///
    /// An error will occur if the current working directory cannot be obtained, or if there is a
    /// failure while starting the sled instance
    pub fn new() -> Result<SledKvsEngine> {
        let cwd = env::current_dir()?;
        let store = Db::start_default(cwd)?;
        Ok(SledKvsEngine { store })
    }
}

impl KvsEngine for SledKvsEngine {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.store.get(key)? {
            Some(value) =>
            match str::from_utf8(value.as_ref()) {
                    Ok(value) => Ok(Some(value.to_owned())),
                    Err(err) => {
                        warn!("Sled UTF8 decode error: {}", err);
                        Err(KvError::UnknownError)
                    }
                }
            None => Ok(None)
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        match self.store.set(key, IVec::from(value.into_bytes())) {
            Ok(_) => Ok(()),
            Err(err) => Err(KvError::SledError(err))
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        match self.store.del(key) {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(KvError::BadRemovalError),
            Err(err) => Err(KvError::SledError(err))
        }
    }
}
