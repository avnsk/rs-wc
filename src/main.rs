use std::fs;

use clap::Parser;
#[derive(Parser, Debug)]
struct Arguments {
    #[arg(short)]
    c: bool,
    file: String,
}
fn main() {
    let args = Arguments::parse();
    let content = match fs::read(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Could not read file '{}'.", args.file);
            eprintln!("Details: {}", e);
            return;
        }
    };
    if args.c {
        let byte_count = content.len();
        println!("{}, {}", byte_count, args.file);
    }
}
