/// A result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur when working with this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// custom error
    #[error("custom: {0}")]
    Custom(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[cfg(feature = "serde")]
    /// [`serde_firestore_value::to_value`] error
    #[error("to value: {0}")]
    ToValue(#[from] serde_firestore_value::Error),
}
