use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{errors::err::ErrTrait, instructions::chunk::Chunk, vm::table::Table};

use super::{err::ValueErr, values::Value};

pub struct Func {
    arity: usize,
    pub chunk: Chunk,
    name: String,
    ip: RefCell<usize>,
}

impl Func {
    pub fn new(name: String, chunk: Chunk) -> Self {
        Func {
            arity: 0,
            chunk,
            name,
            ip: RefCell::new(0),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn dump_globals(env: Rc<RefCell<Table>>) {
        println!("\n\n============== Globals =============\n\n");
        print!("{}", (*env).borrow());
        println!("\n\n====================================\n\n");
    }

    fn dump_stack(stack: Rc<RefCell<Vec<Value>>>) {
        println!("\n\n============== Stack =============\n\n");
        for value in (*stack).borrow().iter() {
            print!("[{}]", value);
        }
        println!("\n\n==================================\n\n");
    }

    pub fn call(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        env: Rc<RefCell<Table>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        stack_offset: usize,
    ) -> Result<Value, Box<dyn ErrTrait>> {
        if (*call_frame).borrow().len() >= 255 {
            return Err(Box::new(ValueErr::new(
                "Call stack exceeded".to_string(),
                format!("{}(..)", self.name),
            )));
        }

        let pre_exec_ip = *self.ip.borrow();
        self.ip.replace(0);

        (*call_frame).borrow_mut().push(self.name.clone());
        let call_frame_size = (*call_frame).borrow().len();

        let code_len = self.chunk.code.len();
        if self.chunk.code.len() > 0 {
            let mut offset;
            loop {
                if *self.ip.borrow() >= code_len {
                    break;
                }
                let instruction = &self.chunk.code[*self.ip.borrow()];
                offset = instruction.eval(
                    stack.clone(),
                    env.clone(),
                    call_frame.clone(),
                    stack_offset,
                )?;
                if offset > 0 {
                    self.ip.replace(offset);
                } else {
                    self.ip.replace_with(|&mut old| old + 1);
                }
                if (*call_frame).borrow().len() < call_frame_size {
                    // since this is an early return, OP_POPN hasn't run yet, so we need
                    // to do the cleanup here
                    let val = Ok((*stack).borrow_mut().pop().unwrap());
                    (*stack).borrow_mut().truncate(stack_offset);
                    self.ip.replace(pre_exec_ip);
                    return val;
                }
            }
            Self::dump_stack(stack.clone());
            Self::dump_globals(env.clone());
        }

        (*call_frame).borrow_mut().pop();
        self.ip.replace(pre_exec_ip);
        Ok(Value::Nil)
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn set_arity(&mut self, arity: usize) {
        self.arity = arity
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
{}
<fn {}>
{}
{}
",
            "-".repeat(self.name.len() + 4),
            self.name,
            "-".repeat(self.name.len() + 4),
            self.chunk
        )
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Fun {}>", self.name)
    }
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.arity != other.arity
    }
}

pub struct Native {
    name: String,
    arity: usize,
    call_: Box<fn(Rc<RefCell<Vec<Value>>>) -> Result<(), Box<dyn ErrTrait>>>,
}

impl Native {
    pub fn new(
        name: String,
        arity: usize,
        call: Box<fn(Rc<RefCell<Vec<Value>>>) -> Result<(), Box<dyn ErrTrait>>>,
    ) -> Self {
        Native {
            name,
            arity,
            call_: call,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn call(&self, stack: Rc<RefCell<Vec<Value>>>) -> Result<(), Box<dyn ErrTrait>> {
        (*self.call_)(stack.clone())
    }
}

impl Debug for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
{}
<fn {}>
{}
{:?}
",
            "-".repeat(self.name.len() + 4),
            self.name,
            "-".repeat(self.name.len() + 4),
            self.call_
        )
    }
}

impl Display for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Fun {}>", self.name)
    }
}

impl PartialEq for Native {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.arity != other.arity
    }
}
