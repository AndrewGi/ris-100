use crate::tis::core::{Core, Error};
use crate::tis::instruction::lines::Line;
use crate::tis::instruction::Instruction;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default, Debug, Hash)]
pub struct Nop;

impl Instruction for Nop {
    fn perform<L: AsRef<[Line]>>(&self, _core: &mut Core<L>) -> Result<(), Error> {
        //NOP
        Ok(())
    }
}
