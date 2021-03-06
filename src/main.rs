mod executor;
mod header;
mod opcodes;
mod section_header;

use clap::Parser;
use executor::Executor;
use header::{FileHeader, MAGIC_NUMBER};
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[clap()]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let bytes =
        fs::read(&args.input_file).expect(&format!("Unable to load file '{}'", args.input_file));

    Executor::execute(&bytes);
}
