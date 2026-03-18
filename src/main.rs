use clap::Parser;
use clap::error::Error;

use std::num::ParseIntError;
use std::path::PathBuf;
use std::fs;

mod cpu;
mod parser;
mod instruction;

use crate::parser::{ParseError, parse};
use crate::cpu::CPU;

/// Little Man Computer Simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Input File Path
    #[arg(short, long)]
    input_file: PathBuf,
}


/// Entry point
fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let Ok(program) = fs::read_to_string(args.input_file) else {
        return Err(ParseError::InvalidString);
    };

    let enc = parse(&program)?;
    let mut cpu = CPU::new();
    cpu.mem_set(enc);
    let _ = cpu.run();
    
    Ok(())
}