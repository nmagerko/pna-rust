use crate::{KvsError, KvsEngine, Result};
use sled::{Db, IVec};
use std::{env, path, str};

/// An implementation of the `sled` library that is compatible with this library's key-value store
/// interface.
pub struct SledKvsEngine {
    store: sled::Db,
}

impl SledKvsEngine {
    /// Creates a new storage instance using `sled` as the storage engine
    ///
    /// # Errors
    ///
    /// An error will occur if the current working directory cannot be obtained. See `open` for
    /// other possible errors.
    pub fn new() -> Result<SledKvsEngine> {
        let cwd = env::current_dir()?;
        SledKvsEngine::open(&cwd)
    }

    /// Creates a new storage instance using `sled` as the storage engine at the given path
    ///
    /// # Errors
    ///
    /// An error will occur if there is a problem starting the `sled` instance at the given path
    pub fn open(path: &path::Path) -> Result<SledKvsEngine> {
        let db_path = path.join(path::Path::new("sled"));
        let store = Db::start_default(db_path)?;
        Ok(SledKvsEngine { store })
    }
}

impl KvsEngine for SledKvsEngine {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.store.get(key)? {
            Some(value) => match str::from_utf8(value.as_ref()) {
                Ok(value) => Ok(Some(value.to_owned())),
                Err(err) => {
                    warn!("Sled UTF8 decode error: {}", err);
                    Err(KvsError::UnknownError)
                }
            },
            None => Ok(None),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.set(key, IVec::from(value.into_bytes()))?;
        self.store.flush()?;
        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let rm_result = self.store.del(key);
        if let Ok(None) = rm_result {
            return Err(KvsError::BadRemovalError);
        }
        if let Err(err) = rm_result {
            return Err(KvsError::SledError(err));
        }
        self.store.flush()?;
        Ok(())
    }
}
