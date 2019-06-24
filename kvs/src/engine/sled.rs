use crate::common::{identify_env, validate_env};
use crate::{KvError, KvsEngine, Result};
use sled::{Db, IVec};
use std::{env, path, str};

const IDENTITY: &str = "sled";

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
    /// An error will occur if the current working directory cannot be obtained, or if there is a
    /// failure while starting the sled instance. If the current directory has been started with
    /// a different engine than this one, an engine mismatch error will be returned.
    pub fn new() -> Result<SledKvsEngine> {
        let cwd = env::current_dir()?;
        if !validate_env(&cwd, IDENTITY)? {
            let path_str = cwd.to_str().unwrap().to_owned();
            return Err(KvError::EngineMismatchError(path_str));
        }
        identify_env(&cwd, IDENTITY)?;

        let db_path = cwd.join(path::Path::new("sled"));
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
                    Err(KvError::UnknownError)
                }
            },
            None => Ok(None),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        match self.store.set(key, IVec::from(value.into_bytes())) {
            Ok(_) => Ok(()),
            Err(err) => Err(KvError::SledError(err)),
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        match self.store.del(key) {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(KvError::BadRemovalError),
            Err(err) => Err(KvError::SledError(err)),
        }
    }
}
