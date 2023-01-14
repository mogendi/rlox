use std::fmt::{Debug, Display};

use crate::errors::err::{ErrTrait, ErrTraitBase};

pub struct ScannerErr {
    message: String,
    line_contents: String,
    line: usize,
    offset: usize,
}

impl ScannerErr {
    pub fn new(message: String, line_contents: String, line: usize, offset: usize) -> Self {
        ScannerErr {
            message,
            line_contents,
            line,
            offset,
        }
    }
}

impl ErrTraitBase for ScannerErr {
    fn raise(&self) {
        println!("{}", self);
    }
}

impl Display for ScannerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let get_offset = || {
            if self.offset < 2 {
                return self.offset;
            }
            self.offset - 2
        };
        write!(
            f,
            "
Line {}: {}
       {}^
       {}------- {}
",
            self.line,
            self.line_contents,
            " ".repeat(get_offset()),
            " ".repeat(get_offset()),
            self.message
        )
    }
}

impl Debug for ScannerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
Line {}: {}
        {}^
        {}------- {}
",
            self.line,
            self.line_contents,
            " ".repeat(self.offset),
            " ".repeat(self.offset),
            self.message
        )
    }
}

pub type ParserErr = ScannerErr;

pub struct GroupErr {
    errs: Vec<Box<dyn ErrTrait>>,
    message: String,
    label: &'static str,
}

impl GroupErr {
    pub fn _new(label: &'static str, message: String, errs: Vec<Box<dyn ErrTrait>>) -> Self {
        GroupErr {
            errs,
            message,
            label,
        }
    }
}

impl ErrTraitBase for GroupErr {
    fn raise(&self) {
        println!("\n{}:::   {}", self.label, self.message);
        println!(
            "{}\n",
            "=".repeat(self.label.len() + self.message.len() + 6)
        );
        for err in &self.errs {
            err.raise()
        }
    }
}

#[derive(Debug)]
pub struct InterpreterErr {
    message: String,
}

impl InterpreterErr {
    pub fn new(message: String) -> Self {
        InterpreterErr { message }
    }
}

impl ErrTraitBase for InterpreterErr {
    fn raise(&self) {
        print!("Interpreter Error:: {}", self.message)
    }
}

impl Display for InterpreterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
