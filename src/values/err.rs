use std::fmt::{Debug, Display};

use crate::errors::err::ErrTraitBase;

#[derive(PartialEq)]
pub struct ValueErr {
    message: String,
    instruction_str: String,
}

impl ErrTraitBase for ValueErr {
    fn raise(&self) {
        println!("{}", self.message);
    }
}

impl ValueErr {
    pub fn new<'a>(message: String, instruction_str: String) -> Self {
        ValueErr {
            message,
            instruction_str,
        }
    }
}

impl Display for ValueErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value Err::: {}: {} ",
            self.instruction_str, self.message
        )
    }
}

impl Debug for ValueErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value Err::: {}: {} ",
            self.instruction_str, self.message
        )
    }
}
