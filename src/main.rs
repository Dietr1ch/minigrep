extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use color_print::cprintln;
use regex::Regex;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    query: String,

    #[arg()]
    file_paths: Vec<String>,

    #[arg(long)]
    keep_going: bool,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    debug!("Args: {:?}", args);

    let highlight = cformat!("<bold><red>{}</red></bold>", &args.query);
    let re = Regex::new(&args.query).unwrap();

    for file_path in args.file_paths {
        debug!("Reading '{}'...", file_path);

        match fs::read_to_string(&file_path) {
            Ok(text) => {
                for (line_number, line) in text.lines().enumerate() {
                    if re.is_match(line) {
                        cprintln!(
                            "<green>{}</green>:<yellow>{}</yellow>| {}",
                            file_path,
                            line_number,
                            re.replace_all(line, &highlight)
                        );
                    }
                }
            }
            _ => {
                error!("Failed to read file '{}'", file_path);
                if !args.keep_going {
                    panic!(
                        "Failed to read file '{}'. Use `--keep_going` to be lenient on errors.",
                        file_path
                    );
                }
            }
        }
    }
}
