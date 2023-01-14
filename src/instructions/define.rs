use std::{
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
pub enum DefinitionScope {
    Global,
    Local(usize),
}

pub struct Define {
    code: InstructionType,
    scope: DefinitionScope,
    operand: String,
}

impl Define {
    pub fn new(scope: DefinitionScope, operand: String) -> Self {
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
        match self.scope {
            DefinitionScope::Global => {
                let current_stack_index = || {
                    if stack.borrow().len() > 0 {
                        return (*stack).borrow().len() - 1;
                    }
                    0
                };
                (*table).borrow_mut().add(
                    self.operand.clone(),
                    stack.borrow()[current_stack_index()].clone(),
                );
            }
            DefinitionScope::Local(_) => {}
        }
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
    scope: DefinitionScope,
}

impl Resolve {
    pub fn new(identifier: String, scope: DefinitionScope) -> Self {
        Resolve {
            code: InstructionType::OP_RESOLVE,
            identifier,
            scope,
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
        match self.scope {
            DefinitionScope::Global => match (*env).borrow().resolve(&self.identifier) {
                Some(val) => {
                    stack.borrow_mut().push(val);
                }
                None => {
                    return Err(Box::new(InstructionErr::new(
                        format!("undefined variable:: {} not found", self.identifier),
                        format!("{}", self.code),
                    )))
                }
            },
            DefinitionScope::Local(stack_idx) => {
                let val = stack.borrow()[stack_idx].clone();
                stack.borrow_mut().push(val);
            }
        }
        Ok(())
    }
}

impl Debug for Resolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resolve {}  @{:?}>", self.identifier, self.scope)
    }
}

impl Display for Resolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} @{:?}      {}",
            self.code, self.scope, self.identifier
        )
    }
}

pub struct Override {
    code: InstructionType,
    identifier: String,
    scope: DefinitionScope,
}

impl Override {
    pub fn new(identifier: String, scope: DefinitionScope) -> Self {
        Override {
            code: InstructionType::OP_OVERRIDE,
            identifier,
            scope,
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
        match self.scope {
            DefinitionScope::Global => {
                match (*env).borrow_mut().override_(self.identifier.clone(), val) {
                    Some(_) => {}
                    None => {
                        return Err(Box::new(InstructionErr::new(
                            format!("undefined variable:: {} not found", self.identifier),
                            format!("{}", self.code),
                        )))
                    }
                }
            }
            DefinitionScope::Local(stack_idx) => {
                let val = stack.borrow_mut().pop().unwrap();
                (*stack).borrow_mut()[stack_idx] = val;
            }
        }
        Ok(())
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
