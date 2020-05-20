use crate::tis::core::{Core, Error};
use crate::tis::instruction::lines::Line;
use crate::tis::instruction::Instruction;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default, Debug, Hash)]
pub struct Neg;

impl Instruction for Neg {
    fn perform<L: AsRef<[Line]>>(&self, core: &mut Core<L>) -> Result<(), Error> {
        core.acc = -core.acc;
        Ok(())
    }
}
