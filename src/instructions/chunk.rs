use std::fmt::{Debug, Display};

use crate::errors::err::ErrTrait;

use super::{
    err::ChunkErr,
    instructions::{Instruction, InstructionType},
};

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<Box<dyn Instruction>>,
    pub count: usize,
    pub capacity: usize,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            count: 0,
            capacity: 0,
            lines: Vec::new(),
        }
    }

    pub fn write_to_chunk(
        &mut self,
        instruction: Box<dyn Instruction>,
        line: usize,
    ) -> Result<(), Box<dyn ErrTrait>> {
        self.code.push(instruction);
        self.count += 1;
        self.capacity = self.code.capacity();
        self.lines.push(line);
        Ok(())
    }

    fn disassemble(&self) -> Vec<InstructionType> {
        let mut instructions = Vec::new();
        for instruction in &self.code {
            instructions.push(instruction.disassemble());
        }
        instructions
    }

    pub fn swap_instructions(
        &mut self,
        origin: usize,
        dest: usize,
    ) -> Result<(), Box<dyn ErrTrait>> {
        let limit = self.code.len() - 1;
        if origin > limit || dest > limit {
            return Err(Box::new(ChunkErr::new(
                format!(
                    "instruction swap failed for the bounds: {} < --- > {}.\nChunk dump: {}",
                    origin, dest, self
                ),
                self.lines.pop().unwrap_or(0),
            )));
        }
        self.code.swap(origin, dest);
        Ok(())
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let line = 1;
        let mut index = 0;
        for inst in &self.code {
            if index != 0 && line == self.lines[index] {
                str = str + &format!("{}  {}", "|", inst) + "\n";
            } else {
                str = str + &format!("{}  {}", self.lines[index], inst) + "\n";
            }
            index += 1;
        }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::{constant::Constant, return_inst::Return},
        values::values::Value,
    };

    use super::*;

    #[test]
    #[allow(unused_must_use)]
    fn test_disassemble() {
        let mut chunk = Chunk::new();
        chunk.write_to_chunk(Box::new(Return::_new()), 1);
        let insts = chunk.disassemble();
        assert_eq!(insts, vec![InstructionType::OP_RETURN]);
    }

    #[test]
    fn test_chunk_display() {
        let mut chunk = Chunk::new();
        chunk
            .write_to_chunk(Box::new(Constant::new(Value::Number(1.0))), 1)
            .unwrap();
        chunk.write_to_chunk(Box::new(Return::_new()), 1).unwrap();
        assert_eq!(format!("{}", chunk), "1  OP_CONST       1\n|  OP_RETURN\n");
        print!("{}", chunk);
    }
}
