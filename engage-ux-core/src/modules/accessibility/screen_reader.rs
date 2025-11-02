use super::announcement::Announcement;

/// Screen reader interface
pub trait ScreenReader {
    /// Announce a message
    fn announce(&mut self, announcement: Announcement);

    /// Stop current announcement
    fn stop(&mut self);

    /// Check if screen reader is enabled
    fn is_enabled(&self) -> bool;
}
