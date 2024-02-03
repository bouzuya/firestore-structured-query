/// A result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur when working with this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// [`serde_firestore_value::to_value`] error
    #[error("to value: {0}")]
    ToValue(#[from] serde_firestore_value::Error),
}
