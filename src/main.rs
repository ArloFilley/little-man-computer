use clap::Parser;

use std::path::PathBuf;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: PathBuf,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


/// Entry point
fn main() {
    let args = Args::parse();

    if let Ok(file) = fs::read_to_string(args.input_file) {
        println!("{file}");
    }

}
