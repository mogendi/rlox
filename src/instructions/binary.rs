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

#[derive(Debug)]
pub enum BinaryOp {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    EQUAL,
    GREATER,
    LESS,
}

pub struct Binary {
    code: InstructionType,
    op: BinaryOp,
}

impl Binary {
    pub fn new(op: BinaryOp) -> Self {
        Binary {
            code: InstructionType::OP_BINARY,
            op,
        }
    }

    pub fn eval_add(&self, left: Value, right: Value) -> Result<Value, Box<dyn ErrTrait>> {
        let raise_type_err = || {
            Box::new(
                InstructionErr::new(
                    format!("{} can only be performed on 2 Numbers/Strings or used for string concatenation", self),
                    format!("{}", self)
                )
            )
        };
        match left {
            Value::Number(lval) => match right {
                Value::Number(rval) => {
                    let res = lval + rval;
                    return Ok(Value::Number(res));
                }
                Value::String(rval) => {
                    let res = format!("{}{}", lval, rval);
                    return Ok(Value::String(res));
                }
                _ => return Err(raise_type_err()),
            },
            Value::String(lval) => match right {
                Value::Number(rval) => {
                    let res = format!("{}{}", lval, rval);
                    return Ok(Value::String(res));
                }
                Value::String(rval) => {
                    let res = format!("{}{}", lval, rval);
                    return Ok(Value::String(res));
                }
                _ => return Err(raise_type_err()),
            },
            _ => return Err(raise_type_err()),
        }
    }

    fn eval_subtract(&self, left: Value, right: Value) -> Result<Value, Box<dyn ErrTrait>> {
        let raise_type_err = || {
            Box::new(InstructionErr::new(
                format!("{} can only be performed on 2 Numbers", self),
                format!("{}", self),
            ))
        };
        match left {
            Value::Number(lval) => match right {
                Value::Number(rval) => {
                    let res = lval - rval;
                    return Ok(Value::Number(res));
                }
                _ => return Err(raise_type_err()),
            },
            _ => return Err(raise_type_err()),
        }
    }

    fn eval_multiply(&self, left: Value, right: Value) -> Result<Value, Box<dyn ErrTrait>> {
        let raise_type_err = || {
            Box::new(InstructionErr::new(
                format!("{} can only be performed on 2 Numbers", self),
                format!("{}", self),
            ))
        };
        match left {
            Value::Number(lval) => match right {
                Value::Number(rval) => {
                    let res = lval * rval;
                    return Ok(Value::Number(res));
                }
                _ => return Err(raise_type_err()),
            },
            _ => return Err(raise_type_err()),
        }
    }

    fn eval_divide(&self, left: Value, right: Value) -> Result<Value, Box<dyn ErrTrait>> {
        let raise_type_err = || {
            Box::new(InstructionErr::new(
                format!("{} can only be performed on 2 Numbers", self),
                format!("{}", self),
            ))
        };
        match left {
            Value::Number(lval) => match right {
                Value::Number(rval) => {
                    let res = lval / rval;
                    return Ok(Value::Number(res));
                }
                _ => return Err(raise_type_err()),
            },
            _ => return Err(raise_type_err()),
        }
    }

    fn eval_greater(&self, left: Value, right: Value) -> Result<Value, Box<dyn ErrTrait>> {
        let raise_type_err = || {
            Box::new(InstructionErr::new(
                format!("{} can only be performed on 2 Numbers", self),
                format!("{}", self),
            ))
        };
        match left {
            Value::Number(lval) => match right {
                Value::Number(rval) => {
                    let res = lval > rval;
                    return Ok(Value::Bool(res));
                }
                _ => return Err(raise_type_err()),
            },
            _ => return Err(raise_type_err()),
        }
    }

    fn eval_less(&self, left: Value, right: Value) -> Result<Value, Box<dyn ErrTrait>> {
        let raise_type_err = || {
            Box::new(InstructionErr::new(
                format!("{} can only be performed on 2 Numbers", self),
                format!("{}", self),
            ))
        };
        match left {
            Value::Number(lval) => match right {
                Value::Number(rval) => {
                    let res = lval < rval;
                    return Ok(Value::Bool(res));
                }
                _ => return Err(raise_type_err()),
            },
            _ => return Err(raise_type_err()),
        }
    }
}

impl InstructionBase for Binary {
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
        let right = stack.borrow_mut().pop().unwrap();
        let left = stack.borrow_mut().pop().unwrap();
        let res = match self.op {
            BinaryOp::ADD => self.eval_add(left, right)?,
            BinaryOp::SUBTRACT => self.eval_subtract(left, right)?,
            BinaryOp::MULTIPLY => self.eval_multiply(left, right)?,
            BinaryOp::DIVIDE => self.eval_divide(left, right)?,
            BinaryOp::EQUAL => Value::Bool(left == right),
            BinaryOp::GREATER => self.eval_greater(left, right)?,
            BinaryOp::LESS => self.eval_less(left, right)?,
        };
        stack.borrow_mut().push(res.clone());
        Ok(0)
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self.op {
            BinaryOp::ADD => "+",
            BinaryOp::DIVIDE => "/",
            BinaryOp::MULTIPLY => "*",
            BinaryOp::SUBTRACT => "-",
            BinaryOp::EQUAL => "==",
            BinaryOp::GREATER => ">",
            BinaryOp::LESS => "<",
        };
        write!(f, "{:?}", op_str)
    }
}

impl Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.op)
    }
}
