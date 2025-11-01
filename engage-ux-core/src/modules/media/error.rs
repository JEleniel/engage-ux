//! Media error types (primary: MediaError)

/// Media loading error
#[derive(Debug, Clone, PartialEq)]
pub enum MediaError {
    /// Unsupported format
    UnsupportedFormat(String),
    /// Invalid data
    InvalidData(String),
    /// Loading failed
    LoadFailed(String),
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaError::UnsupportedFormat(format) => {
                write!(f, "Unsupported format: {}", format)
            }
            MediaError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            MediaError::LoadFailed(msg) => write!(f, "Load failed: {}", msg),
        }
    }
}

impl std::error::Error for MediaError {}
