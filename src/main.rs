use args::LoxArgs;
use structopt::StructOpt;

mod args;
mod compiler;
mod errors;
mod instructions;
mod runners;
mod values;
mod vm;

fn main() {
    let args: LoxArgs = LoxArgs::from_args();
    args.process_req();
}
