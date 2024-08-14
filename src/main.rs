extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use regex::Regex;
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

    let highlight = cformat!("<bold><red>{}</red></bold>", &args.query);
    let re = Regex::new(&args.query).unwrap();

    for file_path in args.file_paths {
        let text = fs::read_to_string(&file_path).expect("Failed to read file");

        for (line_number, line) in text.lines().enumerate() {
            if re.is_match(line) {
                println!(
                    "{}:{}| {}",
                    file_path,
                    line_number,
                    re.replace_all(line, &highlight)
                );
            }
        }
    }
}
