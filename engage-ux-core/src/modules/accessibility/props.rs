use super::aria_live::AriaLive;
use super::aria_role::AriaRole;

/// Accessibility properties for a component
#[derive(Debug, Clone, Default)]
pub struct AccessibilityProps {
    /// ARIA role
    pub role: Option<AriaRole>,
    /// Accessible label
    pub label: Option<String>,
    /// Accessible description
    pub description: Option<String>,
    /// Whether the component is focusable
    pub focusable: bool,
    /// Tab index for keyboard navigation
    pub tab_index: Option<i32>,
    /// Whether the component is expanded (for expandable elements)
    pub expanded: Option<bool>,
    /// Whether the component is checked (for checkboxes, radios)
    pub checked: Option<bool>,
    /// Whether the component is disabled
    pub disabled: bool,
    /// Whether the component is required
    pub required: bool,
    /// Whether the component is read-only
    pub readonly: bool,
    /// ARIA live region setting
    pub live: Option<AriaLive>,
}

impl AccessibilityProps {
    /// Create new accessibility properties
    pub fn new() -> Self {
        Self::default()
    }

    /// Set ARIA role
    pub fn with_role(mut self, role: AriaRole) -> Self {
        self.role = Some(role);
        self
    }

    /// Set accessible label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set accessible description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set focusable
    pub fn with_focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    /// Set tab index
    pub fn with_tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = Some(tab_index);
        self
    }
}
