use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::compiler::{Compiler, FunctionType},
    errors::err::ErrTrait,
    values::{func::Func, values::Value},
};

use super::{natives::load_natives, table::Table};

pub struct VM<'a> {
    // implicit main
    func: &'a Func,
    frames: Rc<RefCell<Vec<String>>>,
    stack: Rc<RefCell<Vec<Value>>>,
    globals: Rc<RefCell<Table>>,
}

impl<'a> VM<'a> {
    pub fn new(func: &'a Func, globals: Rc<RefCell<Table>>) -> Self {
        VM {
            func,
            frames: Rc::new(RefCell::new(Vec::new())),
            stack: Rc::new(RefCell::new(Vec::new())),
            globals,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn ErrTrait>> {
        match self.func.call(
            self.stack.clone(),
            self.globals.clone(),
            self.frames.clone(),
            0,
        ) {
            Ok(_) => {}
            Err(err) => {
                err.raise();
                println!("\nStack Trace: ");
                println!("-----------------");
                for func in (*self.frames).borrow().iter().rev() {
                    println!("<Fun {}>", func);
                }
            }
        }
        Ok(())
    }

    pub fn compile<'b>(
        src: Vec<u8>,
        globals: Rc<RefCell<Table>>,
    ) -> Result<Func, Box<dyn ErrTrait>> {
        let upvalues = Rc::new(RefCell::new(Vec::new()));
        let func = Compiler::compile(src, FunctionType::Script, globals, None, upvalues, None)?;
        Ok(func)
    }

    pub fn interprate(src: Vec<u8>) -> Result<(), Box<dyn ErrTrait>> {
        let globals = Rc::new(RefCell::new(Table::new()));
        load_natives(globals.clone());
        let __main__ = VM::compile(src, globals.clone())?;
        VM::new(&__main__, globals).run()?;
        Ok(())
    }
}
