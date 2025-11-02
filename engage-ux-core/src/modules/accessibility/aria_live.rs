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
