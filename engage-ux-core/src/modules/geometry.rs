//! Geometric primitives used throughout Engage UX.
//!
//! All geometry in Engage UX is defined in terms of a Unit, which is a developer defined
//! scale of pixels set on the renderer. The default is 1 cm = 1 Unit based on the display density,
//! or set to 100 if the density cannot be determined.
//! For example, on a Mac Retina display with approximately 300 PPI, setting the
//! Unit=300 would allow UI elements to be sized in inches directly (1 Unit = 1 inch).
//! Another example, the Point(1.5,2.0 would be 1.5 cm from the left and 2.0 cm from the top
//! of the screen if the Unit is set to 37.8 (96 PPI) and the display density is 96 PPI.
mod border;
mod bounds;
mod circle;
mod direction;
mod ellipse;
mod line;
mod point;
mod polygon;
mod rectangle;
mod text;
mod transform;

pub use border::Border;
pub use bounds::Bounds;
pub use circle::Circle;
pub use direction::Direction;
pub use ellipse::Ellipse;
pub use line::Line;
pub use point::Point;
pub use polygon::{Polygon, Polyline};
pub use rectangle::Rectangle;
pub use text::Text;
pub use transform::{Move, Transform};
