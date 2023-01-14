use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{errors::err::ErrTrait, vm::table::Table};

use super::values::values::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    OP_RETURN,
    OP_CONST,
    OP_UNARY,
    OP_BINARY,
    OP_PRINT,
    OP_POP,
    OP_POPN,
    OP_DEFINE,
    OP_RESOLVE,
    OP_OVERRIDE,
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

pub trait InstructionBase {
    fn disassemble(&self) -> InstructionType;
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        env: Rc<RefCell<Table>>,
    ) -> Result<(), Box<dyn ErrTrait>>;
}

pub trait Instruction: InstructionBase + Display + Debug {}
impl<T> Instruction for T where T: Display + Debug + InstructionBase {}

pub struct Pop {
    code: InstructionType,
}

impl Pop {
    pub fn new() -> Self {
        Pop {
            code: InstructionType::OP_POP,
        }
    }
}

impl InstructionBase for Pop {
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Table>>,
    ) -> Result<(), Box<dyn ErrTrait>> {
        stack.borrow_mut().pop();
        Ok(())
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)
    }
}

impl Display for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)
    }
}

pub struct PopN {
    code: InstructionType,
    n: usize,
}

impl PopN {
    pub fn new(n: usize) -> Self {
        PopN {
            code: InstructionType::OP_POPN,
            n,
        }
    }
}

impl InstructionBase for PopN {
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Table>>,
    ) -> Result<(), Box<dyn ErrTrait>> {
        let n_actual = (*stack).borrow().len().saturating_sub(self.n);
        stack.borrow_mut().truncate(n_actual);
        Ok(())
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for PopN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.n)
    }
}

impl Display for PopN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.n)
    }
}
