use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    compiler::compiler::UpValue,
    instructions::err::InstructionErr,
    values::{obj::Instance, values::Value},
    vm::table::Table,
};

use super::instructions::{InstructionBase, InstructionType};

pub struct Call {
    code: InstructionType,
    args_len: usize,
    line: usize,
    line_contents: String,
}

impl Call {
    pub fn new(args_len: usize, line: usize, line_contents: String) -> Self {
        Call {
            code: InstructionType::OP_CALL,
            args_len,
            line,
            line_contents,
        }
    }
}

impl InstructionBase for Call {
    fn disassemble(&self) -> InstructionType {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        env: Rc<RefCell<Table>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        _: usize,
        _: Rc<RefCell<Vec<UpValue>>>,
        _: usize,
        _: usize,
    ) -> Result<usize, Box<dyn crate::errors::err::ErrTrait>> {
        let func_pos = (*stack)
            .borrow()
            .len()
            .saturating_sub(self.args_len)
            .saturating_sub(1);
        let val = (*stack).borrow_mut().remove(func_pos);
        match val {
            Value::Func(func) => {
                let arity = (*func).arity();
                if arity != self.args_len {
                    return Err(Box::new(InstructionErr::new(
                        format!(
                            "
Line {}: {}
         ^
         -------- Expected {} argument(s) for {} found {}
",
                            self.line, self.line_contents, arity, func, self.args_len
                        ),
                        format!("{}(...)", func.name()),
                    )));
                }
                let offset = (*stack).borrow().len().saturating_sub(self.args_len);
                let val = func.call(stack.clone(), env, call_frame, offset)?;
                (*stack).borrow_mut().push(val);
            }
            Value::Native(func) => {
                let arity = (*func).arity();
                if arity != self.args_len {
                    return Err(Box::new(InstructionErr::new(
                        format!(
                            "
Line {}: {}
         ^
         -------- Expected {} argument for {} found {}
",
                            self.line, self.line_contents, arity, func, self.args_len
                        ),
                        format!("{}(...)", func.name()),
                    )));
                }
                func.call(stack.clone())?;
            }
            Value::Class(class) => {
                let instance = Instance::new(class.clone());
                (*stack).borrow_mut().push(Value::Instance(Rc::new(instance)));
            }
            _ => {
                return Err(Box::new(InstructionErr::new(
                    format!(
                        "
Line {}: {}
        ^
        -------- Call pattern `identifier(...)` only allowed for functions/class initializations/methods, not {}
",
                        self.line, self.line_contents, val
                    ),
                    format!("{}(...)", val),
                )))
            }
        }
        Ok(0)
    }
}

impl Debug for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @[<{} args>]", self.code, self.args_len)
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @(<{} args>)", self.code, self.args_len)
    }
}
