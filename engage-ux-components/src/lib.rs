//! UI Components for Engage UX
//!
//! Provides all UI components including:
//! - Basic components (Label, Text, Icon, Image)
//! - Interactive components (Button, Input, Checkbox, etc.)
//! - Layout components (Container, Card, Table)
//! - Complex components (Menu, Dialog, etc.)

// Informational components
pub mod avatar;
pub mod breadcrumb;
pub mod icon;
pub mod image;
pub mod label;
pub mod line_numbers;
pub mod list;
pub mod progress;
pub mod ruler;
pub mod text;
pub mod tooltip;

// Interactive components
pub mod button;
pub mod carousel;
pub mod checkbox;
pub mod console;
pub mod date_picker;
pub mod link;
pub mod pagination;
pub mod radio;
pub mod select;
pub mod slider;
pub mod text_area;
pub mod text_editor;
pub mod text_input;
pub mod toggle;

// Graphic and Display components
pub mod group;
pub mod video;

// Notification components
pub mod badge;
pub mod banner;
pub mod toast;

// Layout components
pub mod accordion;
pub mod card;
pub mod container;
pub mod table;
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
pub use accordion::{Accordion, AccordionPanel};
pub use avatar::Avatar;
pub use badge::Badge;
pub use banner::Banner;
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use button::Button;
pub use card::Card;
pub use carousel::{Carousel, CarouselItem};
pub use checkbox::Checkbox;
pub use console::{AnsiColor, Console, ConsoleLine};
pub use container::Container;
pub use date_picker::{Date, DatePicker};
pub use dialog::{AlertDialog, ConfirmDialog, FileDialog, Modal};
pub use group::Group;
pub use icon::Icon;
pub use image::Image;
pub use label::Label;
pub use line_numbers::LineNumbers;
pub use link::Link;
pub use list::{List, ListItem};
pub use menu::{Drawer, Dropdown, HamburgerMenu, MenuItem};
pub use pagination::Pagination;
pub use progress::Progress;
pub use radio::{RadioButton, RadioGroup};
pub use ruler::{Ruler, RulerOrientation, RulerUnit};
pub use select::{Select, SelectOption};
pub use slider::Slider;
pub use table::Table;
pub use tabs::{Tab, Tabs};
pub use text::Text;
pub use text_area::TextArea;
pub use text_editor::TextEditor;
pub use text_input::TextInput;
pub use title_menu::{TitleMenu, TitleMenuItem};
pub use toast::Toast;
pub use toggle::Toggle;
pub use tooltip::{Popover, Tooltip};
pub use video::Video;
pub use window::Window;
pub use window_controls::{WindowControlButton, WindowControls};

#[cfg(test)]
mod tests {
	#[test]
	fn components_modules_exist() {
		// Basic smoke test to ensure modules compile
		assert!(true);
	}
}
