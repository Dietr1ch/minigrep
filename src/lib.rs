extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use color_print::cprintln;
use regex::Regex;
use std::error::Error;
use std::fs;

pub fn search_and_print_matches(
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
