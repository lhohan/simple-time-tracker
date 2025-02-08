pub mod cli;
mod parsing;
mod reporting;

pub mod domain;

use domain::{ParseError, RangeDescription, StartDate, TrackingPeriod};
use domain::{PeriodRequested, ReportType};
use parsing::DateRange;
use parsing::Filter;
use reporting::Report;
use std::path::Path;

pub fn run(
    input_path: &Path,
    project_details_selected: Option<String>,
    from_date: Option<StartDate>,
    period: Option<PeriodRequested>,
) -> Result<(), ParseError> {
    let report_type = project_details_selected
        .map(ReportType::ProjectDetails)
        .unwrap_or(ReportType::Projects);

    let filter = create_filter(&report_type, from_date, &period);
    let tracking_result = parsing::process_input(input_path, &filter)?;

    let period_description = period.map(|p| p.period_description());

    let mut result = String::new();

    // Format header
    result.push_str("Time tracking report for ");
    let period_description_str = period_description
        .clone()
        .map(format_period_description)
        .unwrap_or_default();
    result.push_str(period_description_str.as_str());
    result.push_str("\n");

    if let Some(time_report) = tracking_result.time_entries {
        let tracking_result = time_report.period.clone();
        result.push_str(&format_header(&tracking_result));
        result.push_str("\n");

        let report = match report_type {
            ReportType::Projects => Report::new_overview(time_report),
            ReportType::ProjectDetails(project) => Report::new_project_detail(time_report, project),
        };

        result.push_str(report.to_string().as_str());
        // println!("{}", report);
    } else {
        result.push_str("No data found.");
    }
    println!("{}", result);

    // always print warnings
    tracking_result
        .errors
        .iter()
        .for_each(|error| println!("Warning: {}", error));

    Ok(())
}

fn create_filter(
    report_type: &ReportType,
    from_date: Option<StartDate>,
    period: &Option<PeriodRequested>,
) -> Option<Filter> {
    let project_filter = match report_type {
        ReportType::Projects => None,
        ReportType::ProjectDetails(project) => Some(Filter::Project(project.to_string())),
    };
    let from_date_filter = from_date.map(|date| Filter::DateRange(DateRange::new_from_date(date)));
    let period_filter = period
        .clone()
        .map(|period| Filter::DateRange(period.date_range()));

    project_filter
        .into_iter()
        .chain(from_date_filter)
        .chain(period_filter)
        .reduce(|acc, filter| Filter::And(Box::new(acc), Box::new(filter)))
}

fn format_header(period: &TrackingPeriod) -> String {
    format!(
        "{} -> {}",
        period.start.0.format("%Y-%m-%d"),
        period.end.0.format("%Y-%m-%d")
    )
}

fn format_period_description(description: RangeDescription) -> String {
    description.to_string()
}
