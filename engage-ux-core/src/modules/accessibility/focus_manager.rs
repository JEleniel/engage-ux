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
