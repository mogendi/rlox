use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    compiler::compiler::UpValue, errors::err::ErrTrait, values::values::Value, vm::table::Table,
};

use super::instructions::{InstructionBase, InstructionType};

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
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Table>>,
        _: Rc<RefCell<Vec<String>>>,
        _: usize,
        _: Rc<RefCell<Vec<UpValue>>>,
        _: usize,
        _: usize,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        stack.borrow_mut().push(self.operand.clone());
        Ok(0)
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
