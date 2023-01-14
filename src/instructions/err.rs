use std::fmt::{Debug, Display};

use crate::errors::err::ErrTraitBase;

#[derive(PartialEq)]
pub struct InstructionErr {
    message: String,
    instruction_str: String,
}

impl ErrTraitBase for InstructionErr {
    fn raise(&self) {
        println!("{}", self.message);
    }
}

impl InstructionErr {
    pub fn new<'a>(message: String, instruction_str: String) -> Self {
        InstructionErr {
            message,
            instruction_str,
        }
    }
}

impl Display for InstructionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instruction Err::: {}: {} ",
            self.instruction_str, self.message
        )
    }
}

impl Debug for InstructionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instruction Err::: {}: {} ",
            self.instruction_str, self.message
        )
    }
}

pub struct ChunkErr {
    line: usize,
    message: String,
}

impl ChunkErr {
    pub fn new(message: String, line: usize) -> Self {
        ChunkErr { line, message }
    }
}

impl ErrTraitBase for ChunkErr {
    fn raise(&self) {
        println!("Chunk processing err @ Line 1:: {}", self.message)
    }
}

impl Display for ChunkErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk Err:: {}", self.message)
    }
}

impl Debug for ChunkErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk Err:: {}", self.message)
    }
}
