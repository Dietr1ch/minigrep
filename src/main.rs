extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use color_print::cprintln;
use regex::Regex;
use std::error::Error;
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
fn search_and_print_matches(
    re: &Regex,
    highlight: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    debug!("Reading '{}'...", file_path);

    let text = fs::read_to_string(file_path)?;
    for (line_number, line) in text.lines().enumerate() {
        if re.is_match(line) {
            cprintln!(
                "<green>{}</green>:<yellow>{}</yellow>| {}",
                file_path,
                line_number,
                re.replace_all(line, highlight)
            );
        }
    }

    Ok(())
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    debug!("Args: {:?}", args);

    let highlight = cformat!("<bold><red>{}</red></bold>", &args.query);
    let re = Regex::new(&args.query).unwrap();

    for file_path in args.file_paths {
        match search_and_print_matches(&re, &highlight, &file_path) {
            Ok(()) => {
                continue;
            }
            Err(e) => {
                error!("Failed to read file '{}': {}", file_path, e);
                if !args.keep_going {
                    panic!(
                        "\nFailed to read file '{}'. Use `--keep_going` to be lenient on errors.",
                        file_path
                    );
                }
            }
        }
    }
}
