use crate::{KvError, Result};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// The file that stores the last-used engine in a given directory
const ENGINE_IDENTITY_FILE: &str = ".engine";

/// A serializiable representation of a KvEngine command
#[derive(Debug, Serialize, Deserialize)]
pub enum KvRequest {
    /// Representation of getting a value for a given key
    Get {
        /// The key to retrieve
        key: String,
    },
    /// Representation of setting a value for a given key
    Set {
        /// The key to associate
        key: String,
        /// The value to associate
        value: String,
    },
    /// Representation of removing a key-value pair
    Remove {
        /// The key to remove
        key: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
/// A serializable representation of a KvEngine command
pub enum KvResponse {
    /// Representation of a successful Get
    Get {
        /// The retrieved value
        value: Option<String>,
    },
    /// Representation of a successful Set
    Set,
    /// Representation of a successful Remove
    Remove,
    /// Representation of some error
    Error {
        /// The associated error message
        message: String,
    },
}

/// Identifies the given directory as prepared for the identified engine
///
/// # Arguments
///
/// - path - the path that the engine will be started in
/// - identity - the identity of the engine
///
/// # Errors
///
/// If an IO error occurs while attempting to read the files in the given path, an IoError will
/// be returned.
pub fn identify_env(path: &std::path::Path, identity: &str) -> Result<()> {
    let identity_file = std::path::Path::new(ENGINE_IDENTITY_FILE);
    let identity_path = path.join(identity_file);
    std::fs::File::create(identity_path)?.write_all(identity.as_bytes())?;
    Ok(())
}

/// Checks that the given directory has a valid environment for the identified engine
///
/// # Arguments
///
/// - path - the path that the engine will be started in
///
/// # Errors
///
/// If an IO error occurs while attempting to read the files in the given path, an IoError will
/// be returned. If there is a problem decoding the path, an internal error will occur.
pub fn validate_env(path: &std::path::Path, identity: &str) -> Result<bool> {
    let identity_file = std::path::Path::new(ENGINE_IDENTITY_FILE);
    let identity_path = path.join(identity_file);
    if !identity_path.exists() {
        return Ok(true);
    }

    let mut content = Vec::new();
    std::fs::File::open(identity_path)?.read_to_end(&mut content)?;
    match std::str::from_utf8(&content) {
        Ok(engine) => Ok(engine == identity),
        Err(_) => Err(KvError::InternalError("UTF decode error".to_owned())),
    }
}
