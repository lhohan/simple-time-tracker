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
    let filter = create_filter(project_detail_request, from_date);

    let project_detail_request = project_detail_request.map(String::from);
    let report_result = create_report(&content, &filter, project_detail_request);

    match report_result {
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

fn create_filter(
    project_detail_request: Option<&str>,
    from_date: Option<StartDate>,
) -> Option<Filter> {
    let project_filter = project_detail_request.map(|project| Filter::Project(project.to_string()));
    let date_filter = from_date.map(|date| Filter::DateRange(DateRange::new_from_date(date)));

    project_filter
        .into_iter()
        .chain(date_filter)
        .reduce(|acc, filter| Filter::And(Box::new(acc), Box::new(filter)))
}

fn create_report(
    content: &str,
    filter: &Option<Filter>,
    project_detail_request: Option<String>,
) -> Option<(Report, Vec<ParseError>)> {
    parsing::get_entries(content, filter).map(|parse_result| {
        let entries = parse_result.entries().clone();
        let report = match project_detail_request {
            Some(project) => Report::new_project_detail(
                entries,
                project.clone(),
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            ),
            _ => Report::new_overview(
                entries,
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            ),
        };
        (report, parse_result.errors().clone())
    })
}
