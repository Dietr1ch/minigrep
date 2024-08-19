extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use color_print::cprintln;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The search query.
    #[arg()]
    query: String,

    /// Whether to continue after some of the file_paths fail to read.
    #[arg()]
    file_paths: Vec<Box<Path>>,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    debug!("Args: {:?}", args);

    let highlight = cformat!("<bold><red>{}</red></bold>", &args.query);
    let re = Regex::new(&args.query).unwrap();

    for file_path in args.file_paths {
        if !&file_path.is_file() {
            info!("Skipping directory '{:?}'...", &file_path);
            continue;
        }

        debug!("Reading '{}'", &file_path.display());
        let Ok(text) = read_to_string(&file_path) else {
            continue;
        };

        for (line_number, line) in text.lines().enumerate() {
            if re.is_match(line) {
                cprintln!(
                    "<green>{:?}/<bold>{:?}</bold></green>:<yellow>{}</yellow><bold>|</bold> {}",
                    &file_path.parent(),
                    &file_path.file_name(),
                    line_number,
                    re.replace_all(line, &highlight)
                );
            }
        }
    }
}
