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
pub enum UnaryOp {
    Negate,
    Bang,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            UnaryOp::Negate => "-",
            UnaryOp::Bang => "!",
        };
        write!(f, "{}", op_str)
    }
}

pub struct Unary {
    code: InstructionType,
    op: UnaryOp,
}

impl Unary {
    pub fn new(op: UnaryOp) -> Self {
        Unary {
            code: InstructionType::OP_UNARY,
            op,
        }
    }
}

impl InstructionBase for Unary {
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
        let operand = stack.borrow_mut().pop().unwrap();
        match self.op {
            UnaryOp::Negate => match operand {
                Value::Number(number) => {
                    stack.borrow_mut().push(Value::Number(-number));
                }
                _ => {
                    return Err(Box::new(InstructionErr::new(
                        format!("Invalid operand [{}] for {:?}", self.op, operand),
                        format!("{}", self),
                    )))
                }
            },
            UnaryOp::Bang => {
                stack.borrow_mut().push(Value::Bool(!operand.truthy()?));
            }
        }
        Ok(0)
    }

    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }
}

impl Debug for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.code, self.op)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.code, self.op)
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::chunk::Chunk;

    use super::*;

    #[test]
    fn test_negate_display() {
        let mut chunk = Chunk::new();
        chunk
            .write_to_chunk(Box::new(Unary::new(UnaryOp::Negate)), 1)
            .unwrap();
        assert_eq!(format!("{}", chunk), "1  OP_UNARY Negate\n");
        print!("{}", chunk);
    }
}
