use crate::tis::value::Value;
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug, Default)]
pub struct Core {
	pub acc: Value,
	pub bak: Value,
}