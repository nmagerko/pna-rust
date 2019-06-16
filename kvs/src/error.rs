/// Alias for `Result<T, E>` where `E: KvError`
pub type Result<T> = std::result::Result<T, KvError>;

/// An enumeration of possible errors in the `kvs` project
#[derive(Debug, Fail)]
pub enum KvError {
    /// An error occured and no more detail could be provided
    #[fail(display = "an unknown error occured: {}", message)]
    UnknownError { message: String }
}
