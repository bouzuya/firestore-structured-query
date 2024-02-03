/// A result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur when working with this crate.
#[derive(Debug)]
pub struct Error {
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl Error {
    /// Create a new error from a source error.
    pub fn new<E>(source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::from(source.into())
    }
}

impl std::convert::From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self { source }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)
    }
}
