use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::compiler::Compiler,
    errors::err::ErrTrait,
    instructions::{chunk::Chunk, values::values::Value},
};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Rc<RefCell<Vec<Value>>>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn dump_stack(&self) {
        println!("\n\n============== Stack =============\n\n");
        for value in self.stack.borrow().iter() {
            print!("[{}]", value);
        }
        println!("\n\n==================================\n\n");
    }

    pub fn run(&self) -> Result<(), Box<dyn ErrTrait>> {
        for instruction in &self.chunk.code {
            instruction.eval(self.stack.clone())?;
        }
        self.dump_stack();
        Ok(())
    }

    pub fn compile<'b>(src: Vec<u8>) -> Result<Chunk, Box<dyn ErrTrait>> {
        Compiler::compile(src)
    }

    pub fn interprate(src: Vec<u8>) -> Result<(), Box<dyn ErrTrait>> {
        VM::new(&VM::compile(src)?).run()?;
        Ok(())
    }
}
