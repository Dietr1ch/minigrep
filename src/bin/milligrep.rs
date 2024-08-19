extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use color_print::cformat;
use color_print::cprintln;
use regex::Regex;
use std::boxed::Box;
use std::fs::read_to_string;
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
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    debug!("Args: {:?}", args);

    let highlight = cformat!("<bold><red>{}</red></bold>", &args.query);
    let re = Regex::new(&args.query).unwrap();

    args.file_paths
        .iter()
        .flat_map(|filename| {
            read_to_string(filename)
                .unwrap() // Error handling?
                .lines()
                .enumerate()
                .map(|l| (filename.as_ref(), l.0, String::from(l.1)))
                .collect::<Vec<(&Path, usize, String)>>()
        })
        .filter(|l| re.is_match(&l.2))
        .for_each(|(filename, line_number, line)| {
            cprintln!(
                "<green>{}</green>:<yellow>{}</yellow>| {}",
                &filename.display(),
                line_number,
                re.replace_all(&line, &highlight)
            );
        });
}
