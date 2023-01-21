use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    compiler::compiler::UpValue, errors::err::ErrTrait, values::values::Value, vm::table::Table,
};

use super::{
    err::InstructionErr,
    instructions::{InstructionBase, InstructionType},
};

pub struct Set {
    code: InstructionType,
    property: String,
    line: usize,
    line_contents: String,
}

impl Set {
    pub fn new(property: String, line: usize, line_contents: String) -> Self {
        Set {
            code: InstructionType::OP_SET,
            property,
            line,
            line_contents,
        }
    }
}

impl InstructionBase for Set {
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
        let val = (*stack).borrow_mut().pop().unwrap();
        let inst = (*stack).borrow_mut().pop().unwrap();
        match inst {
            Value::Instance(instance) => {
                instance.set_prop(self.property.clone(), val.clone());
                (*stack).borrow_mut().push(val);
            }
            _ => {
                return Err(Box::new(InstructionErr::new(
                    format!(
                        "
Line {}: {}
     ^
     -------- The dot notation is only supported for instances of classes, not `{}`
",
                        self.line, self.line_contents, inst
                    ),
                    format!("{}.{}", inst, self.property),
                )));
            }
        }
        Ok(0)
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.code, self.property)
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.code, self.property)
    }
}

pub struct Get {
    code: InstructionType,
    property: String,
    line: usize,
    line_contents: String,
}

impl Get {
    pub fn new(property: String, line: usize, line_contents: String) -> Self {
        Get {
            code: InstructionType::OP_GET,
            property,
            line,
            line_contents,
        }
    }
}

impl InstructionBase for Get {
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
        let inst = (*stack).borrow_mut().pop().unwrap();
        match inst {
            Value::Instance(instance) => match instance.get_prop(self.property.clone()) {
                Some(val) => {
                    (*stack).borrow_mut().push(val);
                }
                None => {
                    return Err(Box::new(InstructionErr::new(
                        format!(
                            "
Line {}: {}
        ^
        -------- `{}` has not property `{}`
",
                            self.line, self.line_contents, instance, self.property
                        ),
                        format!("{}.{}", instance, self.property),
                    )));
                }
            },
            _ => {
                return Err(Box::new(InstructionErr::new(
                    format!(
                        "
Line {}: {}
     ^
     -------- The dot notation is only supported for instances of classes, not `{}`
",
                        self.line, self.line_contents, inst
                    ),
                    format!("{}.{}", inst, self.property),
                )));
            }
        }
        Ok(0)
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.code, self.property)
    }
}

impl Display for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.code, self.property)
    }
}
