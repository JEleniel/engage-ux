use serde::{Deserialize, Serialize};

/// Layout row container used by the layout system to represent a horizontal
/// stacking/flow of child elements. This type is a minimal placeholder used
/// by higher-level layout algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {}
