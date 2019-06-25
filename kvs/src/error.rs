/// Alias for `Result<T, E>` where `E: KvsError`
pub type Result<T> = std::result::Result<T, KvsError>;

/// An enumeration of possible errors in the `kvs` project
#[derive(Debug, Fail)]
pub enum KvsError {
    /// An error occured and no more detail could be provided
    #[fail(display = "An unknown error occured")]
    UnknownError,
    /// A removal request was made with a non-existent key
    #[fail(display = "Key not found")]
    BadRemovalError,
    /// An error indicating the requested engine is incompatable with the current directory
    #[fail(display = "This engine type is incompatable with the current directory")]
    EngineMismatchError,
    /// An error occured and some clarifying detail is provided
    #[fail(display = "An internal error occured: {}", _0)]
    InternalError(String),
    /// An error occured because the requested initialization path was not a directory
    #[fail(display = "The initialization path must be a directory: {}", _0)]
    BadPathError(String),
    /// An error occured while trying to parse the provided address string
    #[fail(display = "Bad address string: {}", _0)]
    BadAddressError(String),
    /// An error occured due to a `std::io::Error`
    #[fail(display = "An io error occured: {}", _0)]
    IoError(#[cause] std::io::Error),
    /// An error occured while trying to serialize or deserialize a command
    #[fail(display = "A serialization error occured: {}", _0)]
    SerdeError(#[cause] serde_json::Error),
    /// An error occured while using the `sled` engine
    #[fail(display = "A sled error occured: {}", _0)]
    SledError(#[cause] sled::Error),
}

impl From<std::io::Error> for KvsError {
    fn from(err: std::io::Error) -> KvsError {
        KvsError::IoError(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::SerdeError(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::SledError(err)
    }
}
