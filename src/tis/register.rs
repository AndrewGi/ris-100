use core::ops;

/// Integer between -999 to 999 (inclusive)
/// Default is `0`
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash, Default)]
pub struct Value(i16);

impl Value {
	pub const ZERO: Value = Value(0);
	pub const ONE: Value = Value(1);
	/// Clamps value into a range of `[-999, 999]`. Doesn't change the value
	/// if its already in range.
	pub fn new_clamped(value: i16) -> Value {
		Value(Self::clamp(value))
	}/// Clamps value into a range of `[-999, 999]`. Doesn't change the value
	/// if its already in range.
	pub fn clamp(value: i16) -> i16 {
		if value > 999_i16 {
			999_i16
		} else if value < -999_i16 {
			-999_i16
		} else {
			value
		}
	}
}
impl From<Value> for i16 {
	fn from(v: Value) -> Self {
		v.0
	}
}
impl ops::Neg for Value {
	type Output = Value;

	fn neg(self) -> Self::Output {
		Self(-self.0)
	}
}
impl ops::Add<i16> for Value {
	type Output = Value;

	fn add(self, rhs: i16) -> Self::Output {
		Self::new_clamped(rhs + self.0)
	}
}
impl ops::Add<Value> for Value {
	type Output = Value;

	fn add(self, rhs: Value) -> Self::Output {
		self + rhs.0
	}
}
impl ops::Sub<i16> for Value {
	type Output = Value;

	fn sub(self, rhs: i16) -> Self::Output {
		Self::new_clamped(self.0 - rhs)
	}
}
impl ops::Sub<Value> for Value {
	type Output = Value;

	fn sub(self, rhs: Value) -> Self::Output {
		self - rhs.0
	}
}
impl ops::Mul<i16> for Value {
	type Output = Value;

	fn mul(self, rhs: i16) -> Self::Output {
		Self::new_clamped(rhs*self.0)
	}
}
impl ops::Mul<Value> for Value {
	type Output = Value;

	fn mul(self, rhs: Value) -> Self::Output {
		self * rhs.0
	}
}