use std::{
    borrow::Borrow,
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{errors::err::ErrTrait, vm::table::Table};

use super::{
    err::InstructionErr,
    instructions::{InstructionBase, InstructionType},
    values::values::Value,
};

#[derive(Debug)]
pub enum Scope {
    Global,
}

pub struct Define {
    code: InstructionType,
    scope: Scope,
    operand: String,
}

impl Define {
    pub fn new(scope: Scope, operand: String) -> Self {
        Define {
            code: InstructionType::OP_DEFINE,
            scope,
            operand,
        }
    }
}

impl InstructionBase for Define {
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        table: Rc<RefCell<Table>>,
    ) -> Result<(), Box<dyn ErrTrait>> {
        (*table)
            .borrow_mut()
            .add(self.operand.clone(), stack.borrow_mut().pop().unwrap());
        Ok(())
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Define {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Def @{:?} from {}>", self.scope, self.operand)
    }
}

impl Display for Define {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.operand)
    }
}

pub struct Resolve {
    code: InstructionType,
    identifier: String,
}

impl Resolve {
    pub fn new(identifier: String) -> Self {
        Resolve {
            code: InstructionType::OP_RESOLVE,
            identifier,
        }
    }
}

impl InstructionBase for Resolve {
    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        env: Rc<RefCell<Table>>,
    ) -> Result<(), Box<dyn ErrTrait>> {
        match (*env).borrow().resolve(&self.identifier) {
            Some(val) => {
                stack.borrow_mut().push(val);
                return Ok(());
            }
            None => {
                return Err(Box::new(InstructionErr::new(
                    format!("undefined variable:: {} not found", self.identifier),
                    format!("{}", self.code),
                )))
            }
        }
    }
}

impl Debug for Resolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resolve {}>", self.identifier)
    }
}

impl Display for Resolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.identifier)
    }
}

pub struct Override {
    code: InstructionType,
    identifier: String,
}

impl Override {
    pub fn new(identifier: String) -> Self {
        Override {
            code: InstructionType::OP_OVERRIDE,
            identifier,
        }
    }
}

impl InstructionBase for Override {
    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        env: Rc<RefCell<Table>>,
    ) -> Result<(), Box<dyn ErrTrait>> {
        let val = stack.borrow_mut().pop().unwrap();
        match (*env).borrow_mut().override_(self.identifier.clone(), val) {
            Some(_) => return Ok(()),
            None => {
                return Err(Box::new(InstructionErr::new(
                    format!("undefined variable:: {} not found", self.identifier),
                    format!("{}", self.code),
                )))
            }
        }
    }
}

impl Debug for Override {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Override {}>", self.identifier)
    }
}

impl Display for Override {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}       {}", self.code, self.identifier)
    }
}
