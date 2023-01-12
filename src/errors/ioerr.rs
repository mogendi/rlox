use std::{fmt::Display, path::PathBuf};

use super::err::ErrTraitBase;

#[derive(Debug)]
pub struct SrcErr {
    message: String,
    expected_stream: PathBuf,
}

impl SrcErr {
    pub fn new(message: String, expected_stream: PathBuf) -> Self {
        SrcErr {
            message,
            expected_stream,
        }
    }
}

impl ErrTraitBase for SrcErr {
    fn raise(&self) {
        println!(
            "IO Error:: File {}: {}",
            self.expected_stream.display(),
            self.message
        )
    }
}

impl Display for SrcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InpErr {
    message: String,
}

impl InpErr {
    pub fn new(message: String) -> Self {
        InpErr { message }
    }
}

impl ErrTraitBase for InpErr {
    fn raise(&self) {
        print!("IO Error:: Failed to read input stream; {}", self.message)
    }
}

impl Display for InpErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
