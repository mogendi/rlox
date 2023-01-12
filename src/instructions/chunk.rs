use std::fmt::{Debug, Display};

use crate::errors::err::ErrTrait;

use super::instructions::{Instruction, InstructionType};

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
    use crate::instructions::{constant::Constant, return_inst::Return, values::Value};

    use super::*;

    #[test]
    #[allow(unused_must_use)]
    fn test_disassemble() {
        let mut chunk = Chunk::new();
        chunk.write_to_chunk(Box::new(Return::new()), 1);
        let insts = chunk.disassemble();
        assert_eq!(insts, vec![InstructionType::OP_RETURN]);
    }

    #[test]
    fn test_chunk_display() {
        let mut chunk = Chunk::new();
        chunk
            .write_to_chunk(Box::new(Constant::new(Value::Number(1.0))), 1)
            .unwrap();
        chunk.write_to_chunk(Box::new(Return::new()), 1).unwrap();
        assert_eq!(format!("{}", chunk), "1  OP_CONST       1\n|  OP_RETURN\n");
        print!("{}", chunk);
    }
}
