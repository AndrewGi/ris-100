use crate::tis::instruction::lines::{Line, Lines};
use crate::tis::instruction::Operand;
//use crate::tis::port::Ports;
use crate::tis::register::Direction;
use crate::tis::value::Value;

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug, Default)]
pub struct Core<L> {
    pub acc: Value,
    pub bak: Value,
    // Program Counter
    pc: usize,
    pub lines: Lines<L>,
    //pub ports: Ports,
}
impl<L: AsRef<[Line]>> Core<L> {
    pub fn store_operand(&mut self, operand: Operand, value: Value) {
        match operand {
            // ACC stores the value in the ACC register
            Operand::Acc => self.acc = value,
            // NIL discards the value
            Operand::Nil => (),
            // LEFT, RIGHT, etc stores the value in the PORT
            Operand::Port(_) => unimplemented!("need to implement ports"),
        }
    }
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum Error {
    PortInBlock(Direction),
    PortOutBlock(Direction),
}
