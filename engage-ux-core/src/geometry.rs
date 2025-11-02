//! Geometric primitives used throughout Engage UX.
//!
//! All geometry in Engage UX is defined in terms of a Unit, which is a developer defined
//! scale of pixels set on the renderer. The default is 1 cm = 1 Unit based on the display density,
//! or set to 100 if the density cannot be determined.
//! For example, on a Mac Retina display with approximately 300 PPI, setting the
//! Unit=300 would allow UI elements to be sized in inches directly (1 Unit = 1 inch).
mod point;
mod rectangle;
mod unit;

pub use point::Point;
pub use rectangle::Rectangle;
pub use unit::{Unit, UnitError};
