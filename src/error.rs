pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("to value: {0}")]
    ToValue(#[from] serde_firestore_value::Error),
}
