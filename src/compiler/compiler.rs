use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    compiler::{parser::Parser, scanner::Scanner},
    errors::err::ErrTrait,
    instructions::{chunk::Chunk, define::DefinitionScope, instructions::PopN},
    values::{func::Func, values::Value},
    vm::table::Table,
};

use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Script,
    Function(String, u32),
}

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    uninit: bool,
    const_: bool,
}

pub struct Compiler {
    locals: Rc<RefCell<Vec<Local>>>,
    locals_count: usize,
    scope_depth: usize,
    pub type_: FunctionType,
    globals: Rc<RefCell<Table>>,
}

impl Compiler {
    pub fn compile(
        src: Vec<u8>,
        type_: FunctionType,
        globals: Rc<RefCell<Table>>,
    ) -> Result<Func, Box<dyn ErrTrait>> {
        let mut compiler = Compiler {
            locals: Rc::new(RefCell::new(Vec::new())),
            locals_count: 0,
            scope_depth: 0,
            type_: type_.clone(),
            globals,
        };
        let scanner = Scanner::new(src);
        let mut chunk = Chunk::new();
        let parser = Parser::new(&scanner, &mut chunk, &mut compiler)?;
        parser.parse()?;
        match type_ {
            FunctionType::Script => Ok(Func::new("__main__".to_string(), chunk)),
            FunctionType::Function(name, _) => Ok(Func::new(name, chunk)),
        }
    }

    pub fn start_scope(&mut self) -> usize {
        self.scope_depth += 1;
        self.scope_depth
    }

    pub fn end_scope<'b>(
        &mut self,
        chunk: &'b mut Chunk,
        line: usize,
    ) -> Result<usize, Box<dyn ErrTrait>> {
        self.scope_depth -= 1;
        let mut pop_count: usize = 0;
        loop {
            if self.locals_count == 0 {
                break;
            }
            if self.scope_depth + 1 != (*self.locals).borrow()[self.locals_count - 1].depth {
                break;
            }
            (*self.locals).borrow_mut().pop();
            self.locals_count -= 1;
            pop_count += 1;
        }
        chunk.write_to_chunk(Box::new(PopN::new(pop_count)), line)?;
        Ok(self.scope_depth)
    }

    pub fn add_local<'b>(&mut self, local: String, const_: bool) -> DefinitionScope {
        (*self.locals).borrow_mut().push(Local {
            name: local.clone(),
            depth: self.scope_depth,
            uninit: true,
            const_,
        });
        self.locals_count += 1;
        if self.scope_depth == 0 {
            return DefinitionScope::Global;
        }
        DefinitionScope::Local((*self.locals).borrow().len() - 1)
    }

    pub fn scope(&self) -> usize {
        self.scope_depth
    }

    pub fn resolve(&self, ident: &Token) -> Option<DefinitionScope> {
        if self.locals_count == 0 && (*self.globals).borrow().keys() == 0 {
            return None;
        }
        let ident_str = format!("{}", ident);
        for (idx, local) in (*self.locals).borrow().iter().enumerate() {
            if local.name == ident_str {
                if local.uninit {
                    return None;
                }
                match local.depth {
                    0 => return Some(DefinitionScope::Global),
                    _ => return Some(DefinitionScope::Local(idx)),
                }
            }
        }
        match (*self.globals).borrow().exists(&ident_str) {
            true => Some(DefinitionScope::Global),
            false => None,
        }
    }

    fn resolve_idx(&self, ident: &Token) -> Option<usize> {
        if self.locals_count == 0 {
            return None;
        }
        let ident_str = format!("{}", ident);
        for (idx, local) in (*self.locals).borrow().iter().rev().enumerate() {
            if local.name == ident_str {
                if local.uninit {
                    return None;
                }
                return Some(idx);
            }
        }
        None
    }

    pub fn check_const(&self, idx: usize) -> bool {
        (*self.locals).borrow()[idx].const_
    }

    pub fn check_const_from_token(&self, ident: &Token) -> bool {
        match self.resolve_idx(ident) {
            Some(idx) => self.check_const(idx),
            None => false,
        }
    }

    pub fn resolve_in_scope(&self, ident: &Token) -> Option<usize> {
        if self.locals_count == 0 {
            return None;
        }
        let start_scope = self.scope_depth;
        let ident_str = format!("{}", ident);
        for (idx, local) in (*self.locals).borrow().iter().rev().enumerate() {
            if local.depth < start_scope {
                break;
            }
            if local.name == ident_str {
                return Some(idx);
            }
        }
        None
    }

    pub fn mark_latest_init(&self) {
        if self.locals_count > 0 {
            if self.scope_depth == 0 {
                let name = &(*self.locals).borrow_mut()[self.locals_count - 1].name;
                (*self.globals).borrow_mut().add(name.clone(), Value::Nil);
            }
            (*self.locals).borrow_mut()[self.locals_count - 1].uninit = false;
        }
    }

    pub fn globals(&self) -> Rc<RefCell<Table>> {
        self.globals.clone()
    }
}

impl Debug for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Count: {}, Depth: {}, Locals: {:?}",
            self.locals_count,
            self.scope_depth,
            (*self.locals).borrow()
        )
    }
}
