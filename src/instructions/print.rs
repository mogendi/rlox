use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{errors::err::ErrTrait, vm::table::Table};

use super::{
    instructions::{InstructionBase, InstructionType},
    values::values::Value,
};

pub struct Print {
    code: InstructionType,
}

impl Print {
    pub fn new() -> Self {
        Print {
            code: InstructionType::OP_PRINT,
        }
    }
}

impl InstructionBase for Print {
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Table>>,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        println!("{}", stack.borrow_mut().pop().unwrap());
        Ok(0)
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)
    }
}
