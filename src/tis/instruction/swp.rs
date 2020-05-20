use crate::tis::core::{Core, Error};
use crate::tis::instruction::lines::Line;
use crate::tis::instruction::Instruction;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default, Debug, Hash)]
pub struct Swp;

impl Instruction for Swp {
    fn perform<L: AsRef<[Line]>>(&self, core: &mut Core<L>) -> Result<(), Error> {
        core::mem::swap(&mut core.acc, &mut core.bak);
        Ok(())
    }
}
