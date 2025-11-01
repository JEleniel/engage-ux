//! Accessibility implementation. Primary type: `AccessibilityProps`.

/// ARIA role for accessibility
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AriaRole {
    /// Button role
    Button,
    /// Link role
    Link,
    /// Textbox role
    Textbox,
    /// Checkbox role
    Checkbox,
    /// Radio button role
    Radio,
    /// Slider role
    Slider,
    /// List role
    List,
    /// List item role
    ListItem,
    /// Menu role
    Menu,
    /// Menu item role
    MenuItem,
    /// Dialog role
    Dialog,
    /// Alert role
    Alert,
    /// Status role
    Status,
    /// Navigation role
    Navigation,
    /// Main content role
    Main,
    /// Complementary role
    Complementary,
    /// Custom role
    Custom(String),
}

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

/// ARIA live region setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaLive {
    /// Polite - wait for pause in speech
    Polite,
    /// Assertive - interrupt current speech
    Assertive,
    /// Off - don't announce
    Off,
}

/// Focus management for keyboard navigation
#[derive(Debug, Default)]
pub struct FocusManager {
    /// Currently focused component ID
    focused_id: Option<usize>,
    /// Focus history for back navigation
    focus_history: Vec<usize>,
}

impl FocusManager {
    /// Create a new focus manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Set focus to a component
    pub fn set_focus(&mut self, id: usize) {
        if let Some(current) = self.focused_id {
            self.focus_history.push(current);
        }
        self.focused_id = Some(id);
    }

    /// Get currently focused component ID
    pub fn focused(&self) -> Option<usize> {
        self.focused_id
    }

    /// Clear focus
    pub fn clear_focus(&mut self) {
        self.focused_id = None;
    }

    /// Return to previous focus
    pub fn focus_previous(&mut self) -> Option<usize> {
        if let Some(previous) = self.focus_history.pop() {
            self.focused_id = Some(previous);
            Some(previous)
        } else {
            None
        }
    }

    /// Check if a component has focus
    pub fn has_focus(&self, id: usize) -> bool {
        self.focused_id == Some(id)
    }
}

/// Screen reader announcements
#[derive(Debug, Clone)]
pub struct Announcement {
    /// Message to announce
    pub message: String,
    /// Priority of announcement
    pub priority: AnnouncementPriority,
}

/// Announcement priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnouncementPriority {
    /// Low priority - can be interrupted
    Low,
    /// Medium priority
    Medium,
    /// High priority - should not be interrupted
    High,
}

impl Announcement {
    /// Create a new announcement
    pub fn new(message: impl Into<String>, priority: AnnouncementPriority) -> Self {
        Self { message: message.into(), priority }
    }

    /// Create a low priority announcement
    pub fn low(message: impl Into<String>) -> Self {
        Self::new(message, AnnouncementPriority::Low)
    }

    /// Create a medium priority announcement
    pub fn medium(message: impl Into<String>) -> Self {
        Self::new(message, AnnouncementPriority::Medium)
    }

    /// Create a high priority announcement
    pub fn high(message: impl Into<String>) -> Self {
        Self::new(message, AnnouncementPriority::High)
    }
}

/// Screen reader interface
pub trait ScreenReader {
    /// Announce a message
    fn announce(&mut self, announcement: Announcement);

    /// Stop current announcement
    fn stop(&mut self);

    /// Check if screen reader is enabled
    fn is_enabled(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessibility_props() {
        let props = AccessibilityProps::new()
            .with_role(AriaRole::Button)
            .with_label("Click me")
            .with_focusable(true);

        assert_eq!(props.role, Some(AriaRole::Button));
        assert_eq!(props.label, Some("Click me".to_string()));
        assert!(props.focusable);
    }

    #[test]
    fn test_focus_manager() {
        let mut manager = FocusManager::new();
        assert_eq!(manager.focused(), None);

        manager.set_focus(1);
        assert_eq!(manager.focused(), Some(1));
        assert!(manager.has_focus(1));

        manager.set_focus(2);
        assert_eq!(manager.focused(), Some(2));

        manager.focus_previous();
        assert_eq!(manager.focused(), Some(1));
    }

    #[test]
    fn test_announcement() {
        let announcement = Announcement::high("Error occurred");
        assert_eq!(announcement.message, "Error occurred");
        assert_eq!(announcement.priority, AnnouncementPriority::High);
    }

    #[test]
    fn test_aria_roles() {
        let button = AriaRole::Button;
        let link = AriaRole::Link;
        assert_ne!(button, link);
    }
}
