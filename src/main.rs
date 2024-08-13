use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    query: String,

    #[arg()]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let text = fs::read_to_string(args.file_path).expect("Failed to read file");
    println!("Text:\n{}", text);
}
