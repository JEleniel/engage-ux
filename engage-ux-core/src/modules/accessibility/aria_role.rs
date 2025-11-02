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
