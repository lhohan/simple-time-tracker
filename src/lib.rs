pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::ReportType;
use domain::{ParseError, StartDate};
use parsing::DateRange;
use parsing::Filter;
use reporting::Report;
use std::fs::read_to_string;
use std::path::Path;

pub fn run(
    input_path: &Path,
    project_details_selected: Option<String>,
    from_date: Option<StartDate>,
) -> Result<(), ParseError> {
    let file_name = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let content = read_to_string(input_path).map_err(|_| {
        ParseError::ErrorReading(
            input_path
                .to_str()
                .expect("Could not get path to file")
                .to_string(),
        )
    })?;

    let report_type = project_details_selected
        .map(ReportType::ProjectDetails)
        .unwrap_or(ReportType::Projects);

    let filter = create_filter(&report_type, from_date);
    let report_result = create_report(&content, &report_type, &filter, &file_name);

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

fn create_filter(report_type: &ReportType, from_date: Option<StartDate>) -> Option<Filter> {
    let project_filter = match report_type {
        ReportType::Projects => None,
        ReportType::ProjectDetails(project) => Some(Filter::Project(project.to_string())),
    };
    let date_filter = from_date.map(|date| Filter::DateRange(DateRange::new_from_date(date)));

    project_filter
        .into_iter()
        .chain(date_filter)
        .reduce(|acc, filter| Filter::And(Box::new(acc), Box::new(filter)))
}

fn create_report(
    content: &str,
    report_type: &ReportType,
    filter: &Option<Filter>,
    file_name: &str,
) -> Option<(Report, Vec<ParseError>)> {
    parsing::get_entries(content, filter).map(|parse_result| {
        let entries = parse_result.entries().clone();
        let report = match report_type {
            ReportType::ProjectDetails(project) => Report::new_project_detail(
                entries,
                project.clone(),
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            ),
            ReportType::Projects => Report::new_overview(
                entries,
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            ),
        };

        let errors: Vec<_> = parse_result
            .errors()
            .into_iter()
            .map(|e| ParseError::WithFile {
                error: Box::new(e),
                file: file_name.to_string(),
            })
            .collect();

        (report, errors)
    })
}
