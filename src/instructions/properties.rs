use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    compiler::compiler::UpValue, errors::err::ErrTrait, values::values::Value, vm::table::Table,
};

use super::{
    define::DefinitionScope,
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
            Value::Instance(instance) => {
                match instance.get_prop(self.property.clone(), instance.clone()) {
                    Some(val) => {
                        (*stack).borrow_mut().push(val);
                    }
                    None => {
                        return Err(Box::new(InstructionErr::new(
                            format!(
                                "
Line {}: {}
          ^
          -------- `{}` has no property `{}`
",
                                self.line, self.line_contents, instance, self.property
                            ),
                            format!("{}.{}", instance, self.property),
                        )));
                    }
                }
            }
            Value::Class(class) => match class.get_method(self.property.clone()) {
                Some(method) => {
                    (*stack)
                        .borrow_mut()
                        .push(Value::ClassMethod(method.clone()));
                }
                None => {
                    return Err(Box::new(InstructionErr::new(
                        format!(
                            "
Line {}: {}
          ^
          -------- `{}` has no method `{}`
",
                            self.line, self.line_contents, class, self.property
                        ),
                        format!("{}.{}", class, self.property),
                    )));
                }
            },
            _ => {
                return Err(Box::new(InstructionErr::new(
                    format!(
                        "
Line {}: {}
          ^
          -------- Property accesses only supported for classes & instances not `{}`
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

pub struct Inherit {
    code: InstructionType,
    ident: String,
    target: DefinitionScope,
    line: usize,
    line_contents: String,
}

impl Inherit {
    pub fn new(target: DefinitionScope, ident: String, line: usize, line_contents: String) -> Self {
        Inherit {
            code: InstructionType::OP_INHERIT,
            ident,
            target,
            line,
            line_contents,
        }
    }
}

impl InstructionBase for Inherit {
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        globals: Rc<RefCell<Table>>,
        _: Rc<RefCell<Vec<String>>>,
        offset: usize,
        upvalue_stack: Rc<RefCell<Vec<UpValue>>>,
        _: usize,
        _: usize,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        let parent = (*stack).borrow_mut().pop().unwrap();
        let child = match self.target {
            DefinitionScope::Global => (*globals).borrow_mut().resolve(&self.ident).unwrap(),
            DefinitionScope::Local(idx) => (*stack).borrow()[idx.saturating_add(offset)].clone(),
            DefinitionScope::UpValue(idx) => (*upvalue_stack).borrow()[idx].value.clone(),
        };
        match parent.clone() {
            Value::Class(parent_class) => match child.clone() {
                Value::Class(child_class) => {
                    (*child_class).inherit(parent_class);
                }
                _ => {
                    return Err(Box::new(InstructionErr::new(
                        format!(
                            "
Line {}: {}
            ^
            -------- Invalid inherit target: only class can inherit from classes, not `{}`
",
                            self.line, self.line_contents, child
                        ),
                        format!("{} < {}", child, parent),
                    )));
                }
            },
            _ => {
                return Err(Box::new(InstructionErr::new(
                    format!(
                        "
Line {}: {}
          ^
          -------- Can only inherit from classes, not `{}`
",
                        self.line, self.line_contents, parent
                    ),
                    format!("{} < {}", child, parent),
                )));
            }
        }
        Ok(0)
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Inherit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} < {:?}", self.code, self.ident)
    }
}

impl Display for Inherit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} < {:?}", self.code, self.ident)
    }
}
