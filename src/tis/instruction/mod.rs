pub mod add;
pub mod jmp;
pub mod lines;
pub mod neg;
pub mod nop;
pub mod sav;
pub mod swp;

use crate::tis::core::Core;
use crate::tis::instruction::lines::Line;
use crate::tis::register;
use crate::tis::value::Value;

pub enum Src {
    Register(register::Which),
    Value(Value),
}
pub trait Instruction {
    fn perform<L: AsRef<[Line]>>(&self, core: &mut Core<L>) -> Result<(), super::core::Error>;
}

pub enum Instructions {
    //TODO
    Nop,
    //TODO
    Mov,
    //TODO
    Swp,
    //TODO
    Sav,
    //TODO
    Add,
    //TODO
    Sub,
    //TODO
    Neg,
    //TODO
    Jmp,
    //TODO
    Jez,
    //TODO
    Jnz,
    //TODO
    Jgz,
    //TODO
    Jlz,
    //TODO
    Jro,
}
