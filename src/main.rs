use clap::Parser;
#[derive(Parser, Debug)]
struct Arguments {
    #[arg(short)]
    c: bool,
    file: String,
}
fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
