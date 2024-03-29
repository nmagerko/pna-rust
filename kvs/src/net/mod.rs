use serde::{Deserialize, Serialize};

/// A serializiable representation of a KvsEngine command
#[derive(Debug, Serialize, Deserialize)]
pub enum KvsRequest {
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
/// A serializable representation of a KvsEngine command
pub enum KvsResponse {
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

pub use client::KvsClient;
pub use server::KvsServer;

mod client;
mod server;
