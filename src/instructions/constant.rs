use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::errors::err::ErrTrait;

use super::{
    instructions::{InstructionBase, InstructionType},
    values::values::Value,
};

#[derive(Debug)]
pub struct Constant {
    code: InstructionType,
    operand: Value,
}

impl Constant {
    pub fn new(operand: Value) -> Self {
        Constant {
            code: InstructionType::OP_CONST,
            operand,
        }
    }
}

impl InstructionBase for Constant {
    fn eval(&self, stack: Rc<RefCell<Vec<Value>>>) -> Result<Value, Box<dyn ErrTrait>> {
        stack.borrow_mut().push(self.operand.clone());
        Ok(self.operand.clone())
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.operand)
    }
}
