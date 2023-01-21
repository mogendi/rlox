use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{compiler::compiler::UpValue, errors::err::ErrTrait, vm::table::Table};

use crate::values::values::Value;

#[allow(non_camel_case_types)]
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
    OP_JUMP,
    OP_NONE,
    OP_CALL,
    OP_SET,
    OP_GET,
    OP_INHERIT,
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
        call_frame: Rc<RefCell<Vec<String>>>,
        offset: usize,
        upvalue_stack: Rc<RefCell<Vec<UpValue>>>,
        upvalue_offset: usize,
        local_upvalue_len: usize,
    ) -> Result<usize, Box<dyn ErrTrait>>;
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
        _: Rc<RefCell<Vec<String>>>,
        _: usize,
        _: Rc<RefCell<Vec<UpValue>>>,
        _: usize,
        _: usize,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        stack.borrow_mut().pop();
        Ok(0)
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
    // returns either an error or a instruction
    // pointer offset
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
        let n_actual = (*stack).borrow().len().saturating_sub(self.n);
        stack.borrow_mut().truncate(n_actual);
        Ok(0)
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

pub struct None {
    code: InstructionType,
}

impl None {
    pub fn new() -> Self {
        None {
            code: InstructionType::OP_NONE,
        }
    }
}

impl InstructionBase for None {
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
        Ok(0)
    }
}

impl Debug for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)
    }
}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)
    }
}
