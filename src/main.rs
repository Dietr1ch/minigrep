extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use regex::Regex;

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
        match minigrep::search_and_print_matches(&re, &highlight, &file_path) {
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
