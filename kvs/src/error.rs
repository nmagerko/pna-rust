/// Alias for `Result<T, E>` where `E: KvError`
pub type Result<T> = std::result::Result<T, KvError>;

/// An enumeration of possible errors in the `kvs` project
#[derive(Debug, Fail)]
pub enum KvError {
    /// An error occured and no more detail could be provided
    #[fail(display = "an unknown error occured")]
    UnknownError,
    /// An error occured because the requested initialization path was not a directory
    #[fail(display = "the initialization path must be a directory: {}", _0)]
    BadPathError(String),
    /// An error occured while trying to parse the provided address string
    #[fail(display = "bad address string: {}", _0)]
    BadAddressError(String),
    /// A removal request was made with a non-existent key
    #[fail(display = "no such key: {}", _0)]
    BadRemovalError(String),
    /// An error occured due to a `std::io::Error`
    #[fail(display = "an io error occured: {}", _0)]
    IoError(#[cause] std::io::Error),
    /// An error occured while trying to serialize or deserialize a command
    #[fail(display = "a serialization error occured: {}", _0)]
    SerdeError(#[cause] serde_json::Error),
}

impl From<std::io::Error> for KvError {
    fn from(err: std::io::Error) -> KvError {
        KvError::IoError(err)
    }
}

impl From<serde_json::Error> for KvError {
    fn from(err: serde_json::Error) -> KvError {
        KvError::SerdeError(err)
    }
}
