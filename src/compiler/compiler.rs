use crate::{
    compiler::{parser::Parser, scanner::Scanner},
    errors::err::ErrTrait,
    instructions::chunk::Chunk,
};

pub struct Compiler {}

impl Compiler {
    pub fn compile(src: Vec<u8>) -> Result<Chunk, Box<dyn ErrTrait>> {
        let scanner = Scanner::new(src);
        let mut chunk = Chunk::new();
        let parser = Parser::new(&scanner, &mut chunk)?;
        parser.parse()?;
        print!("{}", parser);
        Ok(chunk)
    }
}
