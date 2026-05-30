use std::{
    fs::File,
    io::{self, BufReader, Read},
};

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
    file: Option<String>,
}
fn main() {
    let mut args = Arguments::parse();
    if !args.c && !args.l && !args.w && !args.m {
        args.c = true;
        args.l = true;
        args.w = true;
        args.m = true;
    }
    let stream: Box<dyn Read> = match &args.file {
        Some(file_name) => match File::open(file_name) {
            Ok(file) => Box::new(file),
            Err(e) => {
                eprintln!("Error: Could not open file '{}' Details: {}", file_name, e);
                return;
            }
        },
        None => Box::new(io::stdin()),
    };

    let mut reader = BufReader::new(stream);
    let mut buffer = [0u8; 16384];
    let mut byte_count = 0;
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                let content = &buffer[..bytes_read];
                if args.c {
                    byte_count += content.len();
                }
                if args.l {
                    line_count += content.iter().filter(|&&x| x == b'\n').count();
                }
                if args.w {
                    word_count += content
                        .chunk_by(|a, b| a.is_ascii_whitespace() == b.is_ascii_whitespace())
                        .filter(|chunk| !chunk[0].is_ascii_whitespace())
                        .count();
                }
                if args.m {
                    char_count += content
                        .iter()
                        .filter(|&&byte| (byte & 0xC0) != 0x80)
                        .count();
                }
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e);
                return;
            }
        }
    }

    let mut result = Vec::new();
    if args.c {
        result.push(byte_count.to_string());
    }
    if args.l {
        result.push(line_count.to_string());
    }
    if args.w {
        result.push(word_count.to_string());
    }
    if args.m {
        result.push(char_count.to_string());
    }
    if let Some(file_name) = args.file {
        println!("{} {}", result.join(" "), file_name);
    } else {
        println!("{}", result.join(" "));
    }
}
