use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::compiler::{Compiler, FunctionType},
    errors::err::ErrTrait,
    values::{func::Func, values::Value},
};

use super::table::Table;

pub struct VM<'a> {
    // implicit main
    func: &'a Func,
    frames: Rc<RefCell<Vec<String>>>,
    stack: Rc<RefCell<Vec<Value>>>,
    globals: Rc<RefCell<Table>>,
}

impl<'a> VM<'a> {
    pub fn new(func: &'a Func) -> Self {
        VM {
            func,
            frames: Rc::new(RefCell::new(Vec::new())),
            stack: Rc::new(RefCell::new(Vec::new())),
            globals: Rc::new(RefCell::new(Table::new())),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn ErrTrait>> {
        self.func.call(
            self.stack.clone(),
            self.globals.clone(),
            self.frames.clone(),
            0,
        )?;
        Ok(())
    }

    pub fn compile<'b>(src: Vec<u8>) -> Result<Func, Box<dyn ErrTrait>> {
        Compiler::compile(src, FunctionType::Script)
    }

    pub fn interprate(src: Vec<u8>) -> Result<(), Box<dyn ErrTrait>> {
        VM::new(&VM::compile(src)?).run()?;
        Ok(())
    }
}
