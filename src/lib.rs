extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use color_print::cprintln;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

type StatusOr<T> = Result<T, Box<dyn Error>>;
type Status = StatusOr<()>;

pub fn search_and_print_matches(re: &Regex, highlight: &str, file_path: &Path) -> Status {
    if !file_path.is_file() {
        info!("Skipping directory '{:?}'...", file_path);
        return Ok(());
    }

    debug!("Reading '{:?}'...", file_path);
    let text = fs::read_to_string(file_path)?;

    for (line_number, line) in text.lines().enumerate() {
        if re.is_match(line) {
            cprintln!(
                "<green>{:?}/<bold>{:?}</bold></green>:<yellow>{}</yellow><bold>|</bold> {}",
                file_path.parent(),
                file_path.file_name(),
                line_number,
                re.replace_all(line, highlight)
            );
        }
    }

    Ok(())
}
