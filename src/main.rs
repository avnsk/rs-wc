use std::fs;

use clap::Parser;
#[derive(Parser, Debug)]
struct Arguments {
    #[arg(short)]
    c: bool,
    #[arg(short)]
    l: bool,
    #[arg(short)]
    w: bool,
    #[arg(short)]
    m: bool,
    file: String,
}
fn main() {
    let mut args = Arguments::parse();
    if !args.c && !args.l && !args.w && !args.m {
        args.c = true;
        args.l = true;
        args.w = true;
    }
    let mut result = Vec::new();
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
        result.push(byte_count.to_string());
    }
    if args.l {
        let line_count = content.iter().filter(|&&x| x == b'\n').count();
        result.push(line_count.to_string());
    }
    if args.w {
        let word_count = content
            .chunk_by(|a, b| a.is_ascii_whitespace() == b.is_ascii_whitespace())
            .filter(|chunk| !chunk[0].is_ascii_whitespace())
            .count();
        result.push(word_count.to_string());
    }
    if args.m {
        let char_count = content
            .iter()
            .filter(|&&byte| (byte & 0xC0) != 0x80)
            .count();
        result.push(char_count.to_string());
    }
    println!("{}, {}", result.join(" "), args.file);
}
