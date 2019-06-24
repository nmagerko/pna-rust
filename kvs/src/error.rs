/// Alias for `Result<T, E>` where `E: KvError`
pub type Result<T> = std::result::Result<T, KvError>;

/// An enumeration of possible errors in the `kvs` project
#[derive(Debug, Fail)]
pub enum KvError {
    /// An error occured and no more detail could be provided
    #[fail(display = "an unknown error occured")]
    UnknownError,
    /// A removal request was made with a non-existent key
    #[fail(display = "no such key")]
    BadRemovalError,
    /// An error occured and some clarifying detail is provided
    #[fail(display = "an internal error occured: {}", _0)]
    InternalError(String),
    /// An error indicating the requested engine is incompatable with the current directory
    #[fail(
        display = "this engine type is incompatable with the requested directory ({})",
        _0
    )]
    EngineMismatchError(String),
    /// An error occured because the requested initialization path was not a directory
    #[fail(display = "the initialization path must be a directory: {}", _0)]
    BadPathError(String),
    /// An error occured while trying to parse the provided address string
    #[fail(display = "bad address string: {}", _0)]
    BadAddressError(String),
    /// An error occured due to a `std::io::Error`
    #[fail(display = "an io error occured: {}", _0)]
    IoError(#[cause] std::io::Error),
    /// An error occured while trying to serialize or deserialize a command
    #[fail(display = "a serialization error occured: {}", _0)]
    SerdeError(#[cause] serde_json::Error),
    /// An error occured while using the `sled` engine
    #[fail(display = "a sled error occured: {}", _0)]
    SledError(#[cause] sled::Error),
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

impl From<sled::Error> for KvError {
    fn from(err: sled::Error) -> KvError {
        KvError::SledError(err)
    }
}
