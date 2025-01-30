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
    filter_project: Option<&str>,
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
    let filter_project = filter_project.map(String::from);
    let project_filter: Option<Filter> = { filter_project.clone().map(Filter::Project) };
    let date_filter: Option<Filter> =
        from_date.map(|date| Filter::DateRange(DateRange::new_from_date(date)));
    let filter = match (project_filter, date_filter) {
        (None, None) => None,
        (None, Some(filter)) => Some(filter),
        (Some(filter), None) => Some(filter),
        (Some(filter_1), Some(filter_2)) => {
            Some(Filter::And(Box::new(filter_1), Box::new(filter_2)))
        }
    };

    let parse_result = parsing::get_entries(&content, &filter);

    let entries = parse_result.entries().clone(); // todo: fix clone, make ParseResult return reference or immutable structure?
    let report = if let Some(project) = filter_project {
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
