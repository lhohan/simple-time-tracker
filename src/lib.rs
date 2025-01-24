pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::ParseError;
use reporting::Report;
use std::fs::read_to_string;
use std::path::Path;

pub fn run(input_path: &Path, project_filter: Option<&str>) -> Result<(), ParseError> {
    let content = read_to_string(input_path).map_err(|_| ParseError::InvalidFormat)?;
    let (entries, days) = parsing::get_entries(&content)?;
    let project_filter = project_filter.map(String::from);
    let report = Report::new(entries, days, project_filter);
    report.display();

    Ok(())
}
