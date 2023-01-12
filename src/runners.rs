use crate::errors::err::ErrTrait;
use crate::errors::ioerr::{InpErr, SrcErr};
use crate::vm::vm::VM;
use std::io::stdin;
use std::path::PathBuf;
use std::process;
use std::{fs, io};

pub struct SrcRunner {
    path: PathBuf,
}

impl SrcRunner {
    pub fn new(path: PathBuf) -> Self {
        return SrcRunner { path };
    }

    pub fn execute(&self) {
        let src_file = fs::read(self.path.clone()).unwrap_or_else(|_| {
            (&SrcErr::new(
                format!("Could not find src file: {}", self.path.to_str().unwrap()),
                self.path.clone(),
            ) as &dyn ErrTrait)
                .raise();
            process::exit(1);
        });
        VM::interprate(src_file).unwrap_or_else(|err| err.raise());
    }
}

pub struct InteractiveRunner {}

impl InteractiveRunner {
    pub fn new() -> Self {
        InteractiveRunner {}
    }

    pub fn execute(&self) {
        let mut line: String = String::new();
        print!("The Lox Interpreter\n");
        let mut src = String::new();
        loop {
            if (&src).len() > 0 {
                print!("...  ");
            } else {
                print!(">>>  ");
            }
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            match stdin().read_line(&mut line) {
                Ok(_) => {
                    if line == "\n" && (&src).len() > 0 {
                        VM::interprate(Vec::<u8>::from(src.clone()))
                            .unwrap_or_else(|err| err.raise());
                        src.clear();
                    }
                    if line != "\n" && line != "\r" {
                        src = src + &line;
                    }
                    line.clear();
                }
                Err(err) => (&InpErr::new(err.to_string()) as &dyn ErrTrait).raise(),
            }
        }
    }
}
