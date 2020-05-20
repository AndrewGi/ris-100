use core::ops;

/// Integer between -999 to 999 (inclusive)
/// Default is `0`
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash, Default)]
pub struct Value(i16);

impl Value {
    pub const NIL: Value = Self::ZERO;
    pub const ZERO: Value = Value(0);
    pub const ONE: Value = Value(1);
    /// Clamps value into a range of `[-999, 999]`. Doesn't change the value
    /// if its already in range.
    pub fn new_clamped(value: i16) -> Value {
        Value(Self::clamp(value))
    }
    /// Clamps value into a range of `[-999, 999]`. Doesn't change the value
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
impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
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
impl ops::AddAssign<i16> for Value {
    fn add_assign(&mut self, rhs: i16) {
        *self = Self::new_clamped(self.0 + rhs)
    }
}
impl ops::AddAssign<Value> for Value {
    fn add_assign(&mut self, rhs: Value) {
        *self += rhs.0
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
impl ops::SubAssign<i16> for Value {
    fn sub_assign(&mut self, rhs: i16) {
        *self = *self - rhs
    }
}
impl ops::SubAssign<Value> for Value {
    fn sub_assign(&mut self, rhs: Value) {
        *self -= rhs.0
    }
}
impl ops::Mul<i16> for Value {
    type Output = Value;

    fn mul(self, rhs: i16) -> Self::Output {
        Self::new_clamped(rhs * self.0)
    }
}
impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        self * rhs.0
    }
}
impl ops::MulAssign<i16> for Value {
    fn mul_assign(&mut self, rhs: i16) {
        *self = *self * rhs
    }
}
impl ops::MulAssign<Value> for Value {
    fn mul_assign(&mut self, rhs: Value) {
        *self *= rhs.0
    }
}
