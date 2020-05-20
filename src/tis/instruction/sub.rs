use crate::tis::core::{Core, Error};
use crate::tis::instruction::lines::Line;
use crate::tis::instruction::{Instruction, Operand};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Sub(pub Operand);

impl Instruction for Sub {
    fn perform<L: AsRef<[Line]>>(&self, core: &mut Core<L>) -> Result<(), Error> {
        core.acc -= self.0.get_value(core);
        Ok(())
    }
}
