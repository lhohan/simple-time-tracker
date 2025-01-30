pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::ParseError;
use parsing::Filter;
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
    let project_filter = project_filter.map(String::from);
    let filter: Option<Filter> = { project_filter.clone().map(Filter::Project) };
    let parse_result = parsing::get_entries(&content, &filter);

    let entries = parse_result.entries().clone(); // todo: fix clone, make ParseResult return reference or immutable structure?
    let report = if let Some(project) = project_filter {
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
    println!("{}", report);

    let errors = parse_result.errors();
    errors
        .into_iter()
        .for_each(|error| println!("Warning: {}", error));

    Ok(())
}
