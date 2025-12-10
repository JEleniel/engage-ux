use serde::{Deserialize, Serialize};

/// Layout column container used by the layout system to represent a vertical
/// stacking/flow of child elements. This type is a minimal placeholder used
/// by higher-level layout algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {}
