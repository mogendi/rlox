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
