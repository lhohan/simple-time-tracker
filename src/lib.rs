pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::ParseError;
use reporting::Report;
use std::fs::read_to_string;
use std::path::Path;

pub fn run(input_path: &Path, project_filter: Option<&str>) -> Result<(), ParseError> {
    let content = read_to_string(input_path).map_err(|_| {
        ParseError::ErrorReading(
            input_path
                .to_str()
                .expect("Could not get path to file")
                .to_string(),
        )
    })?;
    let parse_result = parsing::get_entries(&content);

    let entries = parse_result.entries().clone();
    let days = parse_result.days();
    let project_filter = project_filter.map(String::from);
    let report = Report::new(entries, days, project_filter);
    report.display();

    let errors = parse_result.errors();
    errors
        .into_iter()
        .for_each(|error| println!("Warning: {}", error));

    Ok(())
}
