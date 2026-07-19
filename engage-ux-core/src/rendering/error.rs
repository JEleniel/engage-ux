//! Rendering errors (primary: RenderError)

/// Rendering error types
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    /// Invalid SVG syntax
    InvalidSvg(String),
    /// Unsupported SVG feature
    UnsupportedFeature(String),
    /// Script execution attempted (security violation)
    ScriptDetected,
    /// External resource loading not allowed
    ExternalResourceBlocked(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::InvalidSvg(msg) => write!(f, "Invalid SVG: {}", msg),
            RenderError::UnsupportedFeature(feature) => {
                write!(f, "Unsupported SVG feature: {}", feature)
            }
            RenderError::ScriptDetected => {
                write!(f, "Script execution is not allowed for security reasons")
            }
            RenderError::ExternalResourceBlocked(url) => {
                write!(f, "External resource blocked: {}", url)
            }
        }
    }
}

impl std::error::Error for RenderError {}
