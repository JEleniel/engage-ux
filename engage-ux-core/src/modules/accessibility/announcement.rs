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
        Self {
            message: message.into(),
            priority,
        }
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
