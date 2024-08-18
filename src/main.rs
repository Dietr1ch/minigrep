extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use rayon::prelude::*;
use regex::Regex;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    query: String,

    #[arg()]
    file_paths: Vec<Box<Path>>,

    #[arg(long)]
    keep_going: bool,

    #[arg(long)]
    use_rayon: bool,
}

#[inline(never)]
fn rayon(args: &Args, re: &Regex, highlight: &str) {
    args.file_paths.par_iter().for_each(|file_path| {
        match minigrep::search_and_print_matches(re, highlight, file_path) {
            Ok(()) => {}
            Err(e) => {
                error!("Failed to read file '{}': {}", file_path.display(), e);
                if !args.keep_going {
                    panic!(
                        "\nFailed to read file '{}'. Use `--keep_going` to be lenient on errors.",
                        file_path.display()
                    );
                }
            }
        }
    });
}

#[inline(always)]
fn single_thread(args: &Args, re: &Regex, highlight: &str) {
    for file_path in &args.file_paths {
        match minigrep::search_and_print_matches(re, highlight, file_path) {
            Ok(()) => {}
            Err(e) => {
                error!("Failed to read file '{}': {}", file_path.display(), e);
                if !args.keep_going {
                    panic!(
                        "\nFailed to read file '{}'. Use `--keep_going` to be lenient on errors.",
                        file_path.display()
                    );
                }
            }
        }
    }
}

const MAX_SINGLE_THREAD_FILES: usize = 50;

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    debug!("Args: {:?}", args);

    let highlight = cformat!("<bold><red>{}</red></bold>", &args.query);
    let re = Regex::new(&args.query).unwrap();

    let use_rayon = args.use_rayon || args.file_paths.len() > MAX_SINGLE_THREAD_FILES;
    if !use_rayon {
        single_thread(&args, &re, &highlight);
    } else {
        rayon(&args, &re, &highlight);
    }
}
