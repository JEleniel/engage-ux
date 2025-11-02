//! A unit of geometric measurement (wrapper around a floating point multiplier).
//! The geometric system uses this value multiplied by a dots-per-unit (DPU) factor
//! to convert to actual pixel values for rendering.
//! This allows for resolution-independent sizing of UI elements.
//! Note: If the DPU is set to 1 then the Unit directly corresponds to pixels.
//! By default the DPU is set to pixels per centimeter as detected from the display,
//! or set to 100 if the density cannot be determined.
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

/// A unit of geometric measurement (wrapper around a floating point multiplier).
/// The geometric system uses this value multiplied by a dots-per-unit (DPU) factor
/// to convert to actual pixel values for rendering.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Unit(pub f32);

impl Unit {
	/// The minimum representable Unit value
	pub const MIN: Unit = Unit(f32::MIN);
	/// The maximum representable Unit value
	pub const MAX: Unit = Unit(f32::MAX);
}

impl Add for Unit {
	type Output = Unit;

	fn add(self, rhs: Self) -> Self::Output {
		// Primitive-like behaviour: add inner floats and return Unit.
		Unit(self.0 + rhs.0)
	}
}

impl Sub for Unit {
	type Output = Unit;

	fn sub(self, rhs: Self) -> Self::Output {
		Unit(self.0 - rhs.0)
	}
}

impl Mul for Unit {
	type Output = Unit;

	fn mul(self, rhs: Self) -> Self::Output {
		Unit(self.0 * rhs.0)
	}
}

impl Div for Unit {
	type Output = Unit;

	fn div(self, rhs: Self) -> Self::Output {
		Unit(self.0 / rhs.0)
	}
}

impl<T> Add<T> for &Unit
where
	T: Into<Unit> + Clone,
{
	type Output = Unit;

	fn add(self, rhs: T) -> Self::Output {
		let rhs_u = rhs.clone().into().0;
		Unit(self.0 + rhs_u)
	}
}

impl<T> Sub<T> for &Unit
where
	T: Into<Unit> + Clone,
{
	type Output = Unit;

	fn sub(self, rhs: T) -> Self::Output {
		let rhs_u = rhs.clone().into().0;
		Unit(self.0 - rhs_u)
	}
}

impl<T> Mul<T> for &Unit
where
	T: Into<Unit> + Clone,
{
	type Output = Unit;

	fn mul(self, rhs: T) -> Self::Output {
		let rhs_u = rhs.clone().into().0;
		Unit(self.0 * rhs_u)
	}
}

impl<T> Div<T> for &Unit
where
	T: Into<Unit> + Clone,
{
	type Output = Unit;

	fn div(self, rhs: T) -> Self::Output {
		let rhs_u = rhs.clone().into().0;
		Unit(self.0 / rhs_u)
	}
}

impl<T> From<T> for Unit
where
	T: Into<f32>,
{
	fn from(value: T) -> Self {
		Unit(value.into())
	}
}

/// Errors that can occur during unit arithmetic
#[derive(Debug, thiserror::Error)]
pub enum UnitError {
	/// The result of the arithmetic operation exceeds the upper bounds of the target type
	#[error("Unit arithmetic resulted in overflow")]
	Overflow,
	/// The result of the arithmetic operation exceeds the lower bounds of the target type
	#[error("Unit arithmetic resulted in underflow")]
	Underflow,
}
