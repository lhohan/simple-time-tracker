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

/// Run the time tracking report generation
///
/// # Errors
///
/// Returns `ParseError` if:
/// - The input path cannot be read
/// - The input contains invalid date formats
/// - The input contains invalid time formats
/// - The input contains invalid line formats
/// - The requested period is invalid
pub fn run(
    input_path: &Path,
    project_details_selected: Option<String>,
    from_date: Option<StartDate>,
    period: Option<PeriodRequested>,
) -> Result<(), ParseError> {
    let report_type =
        project_details_selected.map_or(ReportType::Projects, ReportType::ProjectDetails);

    let filter = create_filter(&report_type, from_date, &period);
    let tracking_result = parsing::process_input(input_path, &filter)?;

    let period_description = period.map(|p| p.period_description());
    println!("{}", format_header(period_description.as_ref()));

    if let Some(time_report) = tracking_result.time_entries {
        let tracked_interval = time_report.period.clone();
        println!("{}", &format_interval(&tracked_interval));

        let report = match report_type {
            ReportType::Projects => Report::new_overview(time_report),
            ReportType::ProjectDetails(project) => Report::new_project_detail(time_report, &project),
        };

        println!("{report}");
    } else {
        println!("No data found.");
    }

    // always print warnings
    tracking_result
        .errors
        .iter()
        .for_each(|error| println!("Warning: {error}"));

    Ok(())
}

fn format_header(period_description: Option<&RangeDescription>) -> String {
    let mut result = String::new();

    result.push_str("Time tracking report for ");
    let period_description_str = period_description
        .map(format_period_description)
        .unwrap_or_default();
    result.push_str(period_description_str.as_str());
    result.push('\n');
    result
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
        .reduce(Filter::combine)
}

fn format_interval(period: &TrackingPeriod) -> String {
    format!(
        "{} -> {}",
        period.start.0.format("%Y-%m-%d"),
        period.end.0.format("%Y-%m-%d")
    )
}

fn format_period_description(description: &RangeDescription) -> String {
    description.to_string()
}
