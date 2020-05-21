use crate::tis::core::{Core, Error};
use crate::tis::instruction::lines::Line;
use crate::tis::instruction::{Instruction, Operand, OperandValue};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct Mov {
    pub src: OperandValue,
    pub dst: Operand,
}
impl Instruction for Mov {
    fn perform<L: AsRef<[Line]>>(&self, core: &mut Core<L>) -> Result<(), Error> {
        let src = self.src.get_value(core);
        core.store_operand(self.dst, src);
        Ok(())
    }
}
