pub mod cli;
mod domain;
mod parsing;
mod reporting;

use domain::ReportType;
use domain::{ParseError, StartDate};
use parsing::DateRange;
use parsing::Filter;
use std::path::Path;

pub fn run(
    input_path: &Path,
    project_details_selected: Option<String>,
    from_date: Option<StartDate>,
) -> Result<(), ParseError> {
    let report_type = project_details_selected
        .map(ReportType::ProjectDetails)
        .unwrap_or(ReportType::Projects);

    let filter = create_filter(&report_type, from_date);

    match parsing::process_input(input_path, &filter, &report_type)? {
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
