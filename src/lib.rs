pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::ParseError;
use reporting::Report;
use std::fs::read_to_string;
use std::path::Path;

pub fn run(input_path: &Path) -> Result<(), ParseError> {
    let content = read_to_string(input_path).map_err(|_| ParseError::InvalidFormat)?;
    let entries = parsing::get_entries(&content)?;
    let report = Report::new(entries);
    report.display();

    Ok(())
}
