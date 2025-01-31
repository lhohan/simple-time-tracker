pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::{ParseError, StartDate};
use parsing::DateRange;
use parsing::Filter;
use reporting::Report;
use std::fs::read_to_string;
use std::path::Path;

pub fn run(
    input_path: &Path,
    project_detail_request: Option<&str>,
    from_date: Option<StartDate>,
) -> Result<(), ParseError> {
    let content = read_to_string(input_path).map_err(|_| {
        ParseError::ErrorReading(
            input_path
                .to_str()
                .expect("Could not get path to file")
                .to_string(),
        )
    })?;
    // note: something is not quite right with project_detail_request: it serves 2 purposes which may be clearly encoded: selecting the tasks overview for a project + filtering entries
    let filter = project_detail_request
        .map(String::from)
        .clone()
        .map(Filter::Project)
        .into_iter()
        .chain(from_date.map(|date| Filter::DateRange(DateRange::new_from_date(date))))
        .reduce(|acc, filter| Filter::And(Box::new(acc), Box::new(filter)));

    let report = parsing::get_entries(&content, &filter).map(|parse_result| {
        let entries = parse_result.entries().clone(); // todo: fix clone, make ParseResult return reference or immutable structure?
        let report = if let Some(project) = project_detail_request.map(String::from) {
            Report::new_project_detail(
                entries,
                project.to_string(),
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            )
        } else {
            Report::new_overview(
                entries,
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            )
        };
        (report, parse_result.errors().clone())
    });

    match report {
        Some((report, errors)) => {
            println!("{}", report);

            errors
                .into_iter()
                .for_each(|error| println!("Warning: {}", error));
        }
        None => println!("No data found."),
    }

    Ok(())
}
