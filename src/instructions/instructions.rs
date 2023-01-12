use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::errors::err::ErrTrait;

use super::values::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    OP_RETURN,
    OP_CONST,
    OP_UNARY,
    OP_BINARY,
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

pub trait InstructionBase {
    fn disassemble(&self) -> InstructionType;
    fn eval(&self, stack: Rc<RefCell<Vec<Value>>>) -> Result<Value, Box<dyn ErrTrait>>;
}

pub trait Instruction: InstructionBase + Display + Debug {}
impl<T> Instruction for T where T: Display + Debug + InstructionBase {}
