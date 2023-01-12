use std::path::PathBuf;
use structopt::StructOpt;

use crate::runners::{InteractiveRunner, SrcRunner};

#[derive(StructOpt, Debug)]
#[structopt(name = "Lox", about = "The lox interpreter")]
pub struct LoxArgs {
    /// The .lox file that contains lox code
    pub src: Option<PathBuf>,
}

impl LoxArgs {
    pub fn process_req(&self) {
        match self.src.clone() {
            // execute from source
            Some(path) => {
                SrcRunner::new(path).execute();
            }
            // enter interactive mode
            None => {
                InteractiveRunner::new().execute();
            }
        }
    }
}
