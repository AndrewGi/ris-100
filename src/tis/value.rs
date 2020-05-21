use core::ops;
use core::sync::atomic::{AtomicI16, Ordering};
/// Integer between -999 to 999 (inclusive)
/// Default is `0`
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash, Default)]
#[repr(transparent)]
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
    pub const fn inner(self) -> i16 {
        self.0
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
impl From<i16> for Value {
    fn from(i: i16) -> Self {
        Self::new_clamped(i)
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
#[repr(transparent)]
pub struct AtomicValue(AtomicI16);
impl AtomicValue {
    pub const fn new(value: Value) -> AtomicValue {
        AtomicValue(AtomicI16::new(value.0))
    }
    pub fn load(&self, ordering: Ordering) -> Value {
        self.0.load(ordering).into()
    }
    pub fn store(&self, value: Value, ordering: Ordering) {
        self.0.store(value.0, ordering)
    }
    pub fn swap(&self, value: Value, ordering: Ordering) -> Value {
        self.0.swap(value.0, ordering).into()
    }
}
/// Same as an Atomic `Option<Value>`.
#[repr(transparent)]
pub struct AtomicValueOption(AtomicI16);
impl AtomicValueOption {
    /// `Value` is an `i16` with the range of `[-999, 999]` so we use `i16::MIN` (`-32_768_i16`) to
    /// mark a `None` value
    pub const NONE_MARKER: i16 = i16::MIN;
    pub const NONE: AtomicValueOption = AtomicValueOption(AtomicI16::new(0));
    pub fn new(option: Option<Value>) -> AtomicValueOption {
        AtomicValueOption(AtomicI16::new(Self::map_option(option)))
    }
    pub const fn new_value(value: Value) -> AtomicValueOption {
        AtomicValueOption(AtomicI16::new(value.inner()))
    }
    fn map_i16(i: i16) -> Option<Value> {
        if i == Self::NONE_MARKER {
            None
        } else {
            Some(i.into())
        }
    }
    fn map_option(option: Option<Value>) -> i16 {
        option.map(Value::inner).unwrap_or(Self::NONE_MARKER)
    }
    pub fn set_mut(&mut self, option: Option<Value>) {
        *self.0.get_mut() = Self::map_option(option)
    }
    pub fn load(&self, ordering: Ordering) -> Option<Value> {
        Self::map_i16(self.0.load(ordering))
    }
    pub fn store(&self, option: Option<Value>, ordering: Ordering) {
        self.0.store(Self::map_option(option), ordering)
    }
    pub fn store_value(&self, value: Value, ordering: Ordering) {
        self.0.store(value.inner(), ordering)
    }
    pub fn swap(&self, option: Option<Value>, ordering: Ordering) -> Option<Value> {
        Self::map_i16(self.0.swap(Self::map_option(option), ordering))
    }
    pub fn compare_and_swap(
        &self,
        current: Option<Value>,
        new: Option<Value>,
        ordering: Ordering,
    ) -> Option<Value> {
        Self::map_i16(self.0.compare_and_swap(
            Self::map_option(current),
            Self::map_option(new),
            ordering,
        ))
    }
    pub fn compare_exchange(
        &self,
        current: Option<Value>,
        new: Option<Value>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Option<Value>, Option<Value>> {
        self.0
            .compare_exchange(
                Self::map_option(current),
                Self::map_option(new),
                success,
                failure,
            )
            .map(Self::map_i16)
            .map_err(Self::map_i16)
    }
}
