use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{errors::err::ErrTrait, values::values::Value, vm::table::Table};

use super::instructions::{InstructionBase, InstructionType};

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
    fn eval(
        &self,
        _: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Table>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        _: usize,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        (*call_frame).borrow_mut().pop();
        Ok(0)
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
