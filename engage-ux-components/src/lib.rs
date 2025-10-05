//! UI Components for Engage UX
//!
//! Provides all UI components including:
//! - Basic components (Label, Text, Icon, Image)
//! - Interactive components (Button, Input, Checkbox, etc.)
//! - Layout components (Container, Card, Table)
//! - Complex components (Menu, Dialog, etc.)

// Informational components
pub mod label;
pub mod text;
pub mod icon;
pub mod image;
pub mod list;
pub mod progress;
pub mod tooltip;
pub mod breadcrumb;
pub mod avatar;
pub mod line_numbers;
pub mod ruler;

// Interactive components
pub mod button;
pub mod text_input;
pub mod text_area;
pub mod checkbox;
pub mod radio;
pub mod toggle;
pub mod slider;
pub mod select;
pub mod link;
pub mod pagination;
pub mod carousel;
pub mod date_picker;
pub mod text_editor;
pub mod console;

// Graphic and Display components
pub mod group;
pub mod video;

// Notification components
pub mod badge;
pub mod toast;
pub mod banner;

// Layout components
pub mod container;
pub mod card;
pub mod table;
pub mod accordion;
pub mod tabs;
pub mod window;

// Menu components
pub mod menu;
pub mod title_menu;

// Dialog components
pub mod dialog;

// Window components
pub mod window_controls;

// Re-exports
pub use label::Label;
pub use text::Text;
pub use icon::Icon;
pub use image::Image;
pub use button::Button;
pub use text_input::TextInput;
pub use text_area::TextArea;
pub use checkbox::Checkbox;
pub use radio::{RadioButton, RadioGroup};
pub use toggle::Toggle;
pub use slider::Slider;
pub use container::Container;
pub use card::Card;
pub use list::{List, ListItem};
pub use select::{Select, SelectOption};
pub use link::Link;
pub use progress::Progress;
pub use tooltip::{Tooltip, Popover};
pub use badge::Badge;
pub use toast::Toast;
pub use banner::Banner;
pub use table::Table;
pub use window_controls::{WindowControlButton, WindowControls};
pub use accordion::{Accordion, AccordionPanel};
pub use tabs::{Tabs, Tab};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use avatar::Avatar;
pub use menu::{Dropdown, Drawer, HamburgerMenu, MenuItem};
pub use dialog::{AlertDialog, ConfirmDialog, Modal, FileDialog};
pub use pagination::Pagination;
pub use line_numbers::LineNumbers;
pub use ruler::{Ruler, RulerOrientation, RulerUnit};
pub use carousel::{Carousel, CarouselItem};
pub use date_picker::{DatePicker, Date};
pub use text_editor::TextEditor;
pub use console::{Console, ConsoleLine, AnsiColor};
pub use group::Group;
pub use video::Video;
pub use title_menu::{TitleMenu, TitleMenuItem};
pub use window::Window;

#[cfg(test)]
mod tests {
	#[test]
	fn components_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
