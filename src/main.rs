extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    query: String,

    #[arg()]
    file_paths: Vec<String>,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    debug!("Args: {:?}", args);

    for file_path in args.file_paths {
        let text = fs::read_to_string(file_path).expect("Failed to read file");
        println!("Text:\n{}", text);
    }
}
