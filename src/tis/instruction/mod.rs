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
use crate::tis::register::Direction;
use crate::tis::value::Value;

pub enum Src {
    Register(register::Which),
    Value(Value),
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Operand {
    Acc,
    Nil,
    Port(Direction),
}
impl Operand {
    pub fn as_str(self) -> &'static str {
        match self {
            Operand::Acc => "ACC",
            Operand::Nil => "NIL",
            Operand::Port(dir) => dir.as_str(),
        }
    }
}
impl core::fmt::Display for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
pub trait Instruction {
    fn perform<L: AsRef<[Line]>>(&self, core: &mut Core<L>) -> Result<(), super::core::Error>;
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum Tag {
    Nop,
    Mov,
    Swp,
    Sav,
    Add,
    Sub,
    Neg,
    Jmp,
    Jez,
    Jnz,
    Jgz,
    Jlz,
    Jro,
}
impl Tag {
    pub fn as_str(self) -> &'static str {
        match self {
            Tag::Nop => "NOP",
            Tag::Mov => "MOV",
            Tag::Swp => "SWP",
            Tag::Sav => "SAV",
            Tag::Add => "ADD",
            Tag::Sub => "SUB",
            Tag::Neg => "NEG",
            Tag::Jmp => "JMP",
            Tag::Jez => "JEZ",
            Tag::Jnz => "JNZ",
            Tag::Jgz => "JGZ",
            Tag::Jlz => "JLZ",
            Tag::Jro => "JRO",
        }
    }
}
impl core::fmt::Display for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
pub enum Instructions {
    Nop(nop::Nop),
    //TODO
    Mov,
    //TODO
    Swp(swp::Swp),
    //TODO
    Sav(sav::Sav),
    //TODO
    Add,
    //TODO
    Sub,
    //TODO
    Neg(neg::Neg),
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
