use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    compiler::compiler::UpValue, errors::err::ErrTrait, values::values::Value, vm::table::Table,
};

use super::instructions::{InstructionBase, InstructionType};

pub struct Jump {
    code: InstructionType,
    to: usize,
    continue_condition: bool,
}

impl Jump {
    pub fn new(to: usize, continue_condition: bool) -> Self {
        Jump {
            code: InstructionType::OP_JUMP,
            to,
            continue_condition,
        }
    }
}

impl InstructionBase for Jump {
    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }

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
        let idx = stack.borrow().len() - 1;
        let expr_res = stack.borrow_mut()[idx].truthy()?;
        if expr_res == self.continue_condition {
            return Ok(0);
        }
        Ok(self.to)
    }
}

impl Debug for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} to {} if {}",
            self.code, self.to, !self.continue_condition
        )
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}_IF_{}       {}",
            self.code, !self.continue_condition, self.to
        )
    }
}

pub struct ForceJump {
    code: InstructionType,
    to: usize,
}

impl ForceJump {
    pub fn new(to: usize) -> Self {
        ForceJump {
            code: InstructionType::OP_JUMP,
            to,
        }
    }
}

impl InstructionBase for ForceJump {
    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }

    fn eval(
        &self,
        _: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Table>>,
        _: Rc<RefCell<Vec<String>>>,
        _: usize,
        _: Rc<RefCell<Vec<UpValue>>>,
        _: usize,
        _: usize,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        Ok(self.to)
    }
}

impl Debug for ForceJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} to {}", self.code, self.to)
    }
}

impl Display for ForceJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.to)
    }
}
