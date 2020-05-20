use crate::tis::instruction::lines::{Line, Lines};
use crate::tis::value::Value;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug, Default)]
pub struct Core<L> {
    pub acc: Value,
    pub bak: Value,
    // Program Counter
    pc: usize,
    pub lines: Lines<L>,
}
impl Core<L: AsRef<[Line]>> {}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum Error {}
