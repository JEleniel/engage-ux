# Component Development Guide

## Overview

This guide will help you create custom components for Engage UX. Components are the building blocks of the UI and can be simple (like a label) or complex (like a data table).

## Table of Contents

1. [Component Architecture](#component-architecture)
2. [Creating Your First Component](#creating-your-first-component)
3. [Implementing the Component Trait](#implementing-the-component-trait)
4. [Adding Input Handling](#adding-input-handling)
5. [Making Components Accessible](#making-components-accessible)
6. [Theming Support](#theming-support)
7. [Testing Components](#testing-components)
8. [Best Practices](#best-practices)

## Component Architecture

Every component in Engage UX:

- Implements the `Component` trait
- Contains `ComponentProperties` for common attributes (id, visibility, enabled, bounds)
- Is thread-safe using `Arc<RwLock<T>>` wrappers
- Supports event handling via callbacks
- Can implement `InputHandler` for keyboard, mouse, and touch input
- Should have accessibility properties (ARIA roles, labels)

## Creating Your First Component

Let's create a simple "Counter" component as an example.

### Step 1: Define the Component Structure

```rust
use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A simple counter component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
	properties: ComponentProperties,
	count: i32,
	label: String,
	color: Color,
	on_change: Option<Arc<RwLock<dyn Fn(i32) + Send + Sync>>>,
}
```

### Step 2: Implement Constructor and Methods

```rust
impl Counter {
	/// Create a new counter
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			count: 0,
			label: "Count".to_string(),
			color: Color::from_rgb(0, 0, 0),
			on_change: None,
		}
	}

	/// Get current count
	pub fn count(&self) -> i32 {
		self.count
	}

	/// Set count
	pub fn set_count(&mut self, count: i32) {
		self.count = count;
		self.trigger_change();
	}

	/// Increment counter
	pub fn increment(&mut self) {
		self.count += 1;
		self.trigger_change();
	}

	/// Decrement counter
	pub fn decrement(&mut self) {
		self.count -= 1;
		self.trigger_change();
	}

	/// Get label
	pub fn label(&self) -> &str {
		&self.label
	}

	/// Set label
	pub fn set_label(&mut self, label: impl Into<String>) {
		self.label = label.into();
	}

	/// Set change callback
	pub fn on_change<F>(&mut self, callback: F)
	where
		F: Fn(i32) + Send + Sync + 'static,
	{
		self.on_change = Some(Arc::new(RwLock::new(callback)));
	}

	/// Trigger change event
	fn trigger_change(&self) {
		if let Some(callback) = &self.on_change {
			if let Ok(cb) = callback.try_read() {
				cb(self.count);
			}
		}
	}
}
```

### Step 3: Implement the Component Trait

```rust
impl Component for Counter {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn is_visible(&self) -> bool {
		self.properties.visible
	}

	fn set_visible(&mut self, visible: bool) {
		self.properties.visible = visible;
	}

	fn is_enabled(&self) -> bool {
		self.properties.enabled
	}

	fn set_enabled(&mut self, enabled: bool) {
		self.properties.enabled = enabled;
	}

	fn bounds(&self) -> engage_ux_core::component::Rect {
		self.properties.bounds
	}

	fn set_bounds(&mut self, bounds: engage_ux_core::component::Rect) {
		self.properties.bounds = bounds;
	}
}
```

## Implementing the Component Trait

The `Component` trait requires these methods:

- `id()` - Returns the unique component identifier
- `is_visible()` / `set_visible()` - Controls visibility
- `is_enabled()` / `set_enabled()` - Controls enabled state
- `bounds()` / `set_bounds()` - Manages position and size

Most components store these in a `ComponentProperties` field and delegate to it.

## Adding Input Handling

To handle keyboard, mouse, or touch input, implement the `InputHandler` trait:

```rust
use engage_ux_core::input::{InputHandler, KeyboardEvent, MouseEvent, TouchEvent};

impl InputHandler for Counter {
	fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
		use engage_ux_core::input::KeyCode;
		
		if !self.is_enabled() {
			return false;
		}

		match event.key_code {
			KeyCode::ArrowUp => {
				self.increment();
				true // Event handled
			}
			KeyCode::ArrowDown => {
				self.decrement();
				true // Event handled
			}
			_ => false, // Event not handled
		}
	}

	fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
		use engage_ux_core::input::MouseButton;
		
		if !self.is_enabled() {
			return false;
		}

		// Handle mouse clicks on the component
		if let engage_ux_core::input::MouseEvent::ButtonDown { button, x, y } = event {
			if *button == MouseButton::Left {
				if self.bounds().contains_point(*x, *y) {
					self.increment();
					return true;
				}
			}
		}
		
		false
	}
}
```

## Making Components Accessible

All components should support accessibility:

```rust
use engage_ux_core::accessibility::{AccessibilityProps, AriaRole};

impl Counter {
	/// Get accessibility properties
	pub fn accessibility(&self) -> AccessibilityProps {
		AccessibilityProps::new(self.id())
			.with_role(AriaRole::Button)
			.with_label(format!("{}: {}", self.label, self.count))
			.with_description("Use arrow keys or click to change")
	}
}
```

## Theming Support

Components should support theming by accepting color and style parameters:

```rust
impl Counter {
	/// Set text color
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	/// Get text color
	pub fn color(&self) -> Color {
		self.color
	}

	/// Apply theme colors
	pub fn apply_theme(&mut self, theme: &engage_ux_themes::Theme) {
		self.color = theme.colors.primary;
	}
}
```

## Testing Components

Write comprehensive tests for your components:

```rust
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_counter_creation() {
		let counter = Counter::new(1);
		assert_eq!(counter.id(), 1);
		assert_eq!(counter.count(), 0);
		assert!(counter.is_enabled());
		assert!(counter.is_visible());
	}

	#[test]
	fn test_counter_increment() {
		let mut counter = Counter::new(1);
		counter.increment();
		assert_eq!(counter.count(), 1);
		counter.increment();
		assert_eq!(counter.count(), 2);
	}

	#[test]
	fn test_counter_decrement() {
		let mut counter = Counter::new(1);
		counter.decrement();
		assert_eq!(counter.count(), -1);
	}

	#[test]
	fn test_counter_set_count() {
		let mut counter = Counter::new(1);
		counter.set_count(42);
		assert_eq!(counter.count(), 42);
	}

	#[test]
	fn test_counter_callback() {
		use std::sync::Arc;
		use std::sync::atomic::{AtomicI32, Ordering};
		
		let mut counter = Counter::new(1);
		let value = Arc::new(AtomicI32::new(0));
		let value_clone = value.clone();
		
		counter.on_change(move |count| {
			value_clone.store(count, Ordering::SeqCst);
		});
		
		counter.increment();
		// Note: In async context, you'd need to wait for callback
		// This is a simplified example
	}

	#[test]
	fn test_counter_input_handler() {
		use engage_ux_core::input::{KeyCode, KeyModifiers};
		
		let mut counter = Counter::new(1);
		let event_up = KeyboardEvent::key_down(KeyCode::ArrowUp, KeyModifiers::empty());
		let event_down = KeyboardEvent::key_down(KeyCode::ArrowDown, KeyModifiers::empty());
		
		assert!(counter.handle_keyboard(&event_up));
		assert_eq!(counter.count(), 1);
		
		assert!(counter.handle_keyboard(&event_down));
		assert_eq!(counter.count(), 0);
	}
}
```

## Best Practices

### 1. Use Builder Pattern for Complex Components

```rust
impl Counter {
	pub fn builder(id: ComponentId) -> CounterBuilder {
		CounterBuilder::new(id)
	}
}

pub struct CounterBuilder {
	counter: Counter,
}

impl CounterBuilder {
	pub fn new(id: ComponentId) -> Self {
		Self {
			counter: Counter::new(id),
		}
	}

	pub fn label(mut self, label: impl Into<String>) -> Self {
		self.counter.set_label(label);
		self
	}

	pub fn initial_count(mut self, count: i32) -> Self {
		self.counter.set_count(count);
		self
	}

	pub fn color(mut self, color: Color) -> Self {
		self.counter.set_color(color);
		self
	}

	pub fn build(self) -> Counter {
		self.counter
	}
}
```

### 2. Keep Components Focused

Each component should have a single, well-defined purpose. Don't create "god components" that do everything.

### 3. Make Components Composable

Design components to work well together. Use container components to combine simpler components.

### 4. Document Public APIs

Use doc comments for all public methods:

```rust
/// Creates a new counter with the specified ID.
///
/// # Arguments
///
/// * `id` - Unique identifier for the component
///
/// # Examples
///
/// ```
/// let counter = Counter::new(1);
/// assert_eq!(counter.count(), 0);
/// ```
pub fn new(id: ComponentId) -> Self {
	// ...
}
```

### 5. Handle Edge Cases

Always validate input and handle edge cases:

```rust
pub fn set_count(&mut self, count: i32) {
	// Clamp to reasonable range if needed
	self.count = count.clamp(-1000, 1000);
	self.trigger_change();
}
```

### 6. Thread Safety

Components are often used in multi-threaded contexts. Use appropriate synchronization:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub type CounterRef = Arc<RwLock<Counter>>;
```

### 7. Consistent Naming

Follow Rust naming conventions:

- Types use `PascalCase`
- Functions and variables use `snake_case`
- Constants use `SCREAMING_SNAKE_CASE`

### 8. Error Handling

Return `Result` for operations that can fail:

```rust
pub fn load_from_file(&mut self, path: &str) -> Result<(), std::io::Error> {
	// Load configuration
	Ok(())
}
```

## Component Template

Here's a template to start from:

```rust
use engage_ux_core::Color;
use engage_ux_core::component::{Component, ComponentId, ComponentProperties, Rect};
use engage_ux_core::input::{InputHandler, KeyboardEvent, MouseEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyComponent {
	properties: ComponentProperties,
	// Add your component-specific fields here
}

impl MyComponent {
	pub fn new(id: ComponentId) -> Self {
		Self {
			properties: ComponentProperties::new(id),
			// Initialize your fields
		}
	}
	
	// Add your component-specific methods
}

impl Component for MyComponent {
	fn id(&self) -> ComponentId {
		self.properties.id
	}

	fn is_visible(&self) -> bool {
		self.properties.visible
	}

	fn set_visible(&mut self, visible: bool) {
		self.properties.visible = visible;
	}

	fn is_enabled(&self) -> bool {
		self.properties.enabled
	}

	fn set_enabled(&mut self, enabled: bool) {
		self.properties.enabled = enabled;
	}

	fn bounds(&self) -> Rect {
		self.properties.bounds
	}

	fn set_bounds(&mut self, bounds: Rect) {
		self.properties.bounds = bounds;
	}
}

impl InputHandler for MyComponent {
	fn handle_keyboard(&mut self, event: &KeyboardEvent) -> bool {
		// Implement keyboard handling
		false
	}

	fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
		// Implement mouse handling
		false
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_creation() {
		let component = MyComponent::new(1);
		assert_eq!(component.id(), 1);
		assert!(component.is_visible());
		assert!(component.is_enabled());
	}
	
	// Add more tests
}
```

## Next Steps

- Read the [Architecture Documentation](../design/architecture/README.md)
- Explore existing components in `engage-ux-components/src`
- Check out the [examples](../../engage-ux-components/examples)
- Review the [NFRs](../design/architecture/NFRs.md) for requirements

## Getting Help

- Check the [API documentation](https://docs.rs/engage-ux)
- Look at similar existing components
- Read the component architecture docs
- Open an issue on GitHub
