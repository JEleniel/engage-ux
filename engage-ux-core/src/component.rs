//! Component and drawing traits for Engage UX
//!
//! Provides the `Component` trait previously defined in the components
//! crate and a lightweight, backend-agnostic `Drawable` abstraction plus a
//! `DrawableContext` that backends can implement to accept draw commands from
//! components.

use crate::component_properties::ComponentProperties;
use crate::geometry::{Circle, Ellipse, Line, Point, Polygon, Polyline, Rectangle, Text};

/// Stable identifier for components across the workspace
pub type ComponentId = u128;

/// Component trait implemented by UI components.
pub trait Component {
	/// Return the stable id for this component
	fn id(&self) -> ComponentId;

	/// Return a reference to the component properties
	fn properties(&self) -> &ComponentProperties;

	/// Return a mutable reference to the component properties
	fn properties_mut(&mut self) -> &mut ComponentProperties;

	/// Convenience: whether the component is visible
	fn is_visible(&self) -> bool {
		self.properties().visible
	}

	/// Convenience: set the component visible state
	fn set_visible(&mut self, visible: bool) {
		self.properties_mut().visible = visible;
	}

	/// Convenience: whether the component is enabled (accepts input)
	fn is_enabled(&self) -> bool {
		self.properties().enabled
	}

	/// Convenience: set the component enabled state
	fn set_enabled(&mut self, enabled: bool) {
		self.properties_mut().enabled = enabled;
	}
}

/// Backend-agnostic drawing context used by `Drawable` implementations.
///
/// Backends (for example the OAL) should implement this trait allowing
/// components to issue drawing commands without depending on the backend
/// crate directly.
pub trait DrawableContext {
	/// Draw a single point at the given coordinates.
	fn draw_point(&mut self, p: Point);

	/// Draw a straight line segment.
	fn draw_line(&mut self, l: Line);

	/// Draw an axis-aligned rectangle.
	fn draw_rectangle(&mut self, r: Rectangle);

	/// Draw a circle.
	fn draw_circle(&mut self, c: Circle);

	/// Draw an ellipse.
	fn draw_ellipse(&mut self, e: Ellipse);

	/// Draw a closed polygon using the provided points.
	fn draw_polygon(&mut self, p: Polygon);

	/// Draw an open polyline using the provided points.
	fn draw_polyline(&mut self, p: Polyline);

	/// Draw text using the provided `Text` primitive.
	fn draw_text(&mut self, t: Text);
}

/// A component that knows how to draw itself into a `DrawableContext`.
pub trait Drawable {
	/// Draw the component into the provided context.
	fn draw(&self, ctx: &mut dyn DrawableContext);
}
