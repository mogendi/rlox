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
pub struct Return {
    code: InstructionType,
}

impl Return {
    pub fn new() -> Self {
        Return {
            code: InstructionType::OP_RETURN,
        }
    }
}

impl InstructionBase for Return {
    fn eval(&self, stack: Rc<RefCell<Vec<Value>>>) -> Result<Value, Box<dyn ErrTrait>> {
        Ok(Value::Number(0.0))
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.code, f)
    }
}
