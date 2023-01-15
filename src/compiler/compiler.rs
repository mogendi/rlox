use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    compiler::{parser::Parser, scanner::Scanner},
    errors::err::ErrTrait,
    instructions::{chunk::Chunk, define::DefinitionScope, instructions::PopN},
};

use super::token::Token;

#[derive(Debug)]
pub struct Local<'a> {
    name: Token<'a>,
    pub depth: usize,
    uninit: bool,
    const_: bool,
}

pub struct Compiler<'a> {
    locals: Rc<RefCell<Vec<Local<'a>>>>,
    locals_count: usize,
    scope_depth: usize,
}

impl<'a> Compiler<'a> {
    pub fn compile(src: Vec<u8>) -> Result<Chunk, Box<dyn ErrTrait>> {
        let mut compiler = Compiler {
            locals: Rc::new(RefCell::new(Vec::new())),
            locals_count: 0,
            scope_depth: 0,
        };
        let scanner = Scanner::new(src);
        let mut chunk = Chunk::new();
        let parser = Parser::new(&scanner, &mut chunk, &mut compiler)?;
        parser.parse()?;
        print!("{}", parser);
        Ok(chunk)
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
        let pre_drop_len = self.locals_count;
        loop {
            if self.locals_count == 0 {
                break;
            }
            if self.scope_depth <= (*self.locals).borrow()[self.locals_count - 1].depth {
                break;
            }
            (*self.locals).borrow_mut().pop();
            self.locals_count -= 1;
        }
        chunk.write_to_chunk(Box::new(PopN::new(pre_drop_len - self.locals_count)), line)?;
        Ok(self.scope_depth)
    }

    pub fn add_local<'b>(&mut self, local: &'b Token<'a>, const_: bool) -> DefinitionScope {
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
        if self.locals_count == 0 {
            return None;
        }
        let ident_str = format!("{}", ident);
        for (idx, local) in (*self.locals).borrow().iter().rev().enumerate() {
            if format!("{}", local.name) == ident_str {
                if local.uninit {
                    return None;
                }
                match local.depth {
                    0 => return Some(DefinitionScope::Global),
                    _ => return Some(DefinitionScope::Local(idx))
                }
            }
        }
        None
    }

    fn resolve_idx(&self, ident: &Token) -> Option<usize> {
        if self.locals_count == 0 {
            return None;
        }
        let ident_str = format!("{}", ident);
        for (idx, local) in (*self.locals).borrow().iter().rev().enumerate() {
            if format!("{}", local.name) == ident_str {
                if local.uninit {
                    return None;
                }
                return Some(idx)
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
            if format!("{}", local.name) == ident_str {
                return Some(idx);
            }
        }
        None
    }

    pub fn mark_latest_init(&self) {
        if self.locals_count > 0 {
            (*self.locals).borrow_mut()[self.locals_count - 1].uninit = false;
        }
    }
}

impl<'a> Debug for Compiler<'a> {
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
