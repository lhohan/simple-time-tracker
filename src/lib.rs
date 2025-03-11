pub mod cli;
mod parsing;
mod reporting;

pub mod domain;

use domain::reports::OutputLimit;
use domain::DateRange;
use reporting::Report;

use crate::domain::dates::StartDate;
use crate::domain::ParseError;
use crate::domain::RangeDescription;
use crate::domain::{PeriodRequested, TrackingPeriod};
use crate::parsing::filter::Filter;
use crate::reporting::format::Formatter;
use crate::reporting::ReportTypeRequested;
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
    exclude_tags: Vec<String>,
    from_date: Option<StartDate>,
    period: Option<PeriodRequested>,
    limit: Option<OutputLimit>,
    formatter: Box<dyn Formatter>,
) -> Result<(), ParseError> {
    let report_type = project_details_selected.clone().map_or(
        ReportTypeRequested::Overview,
        ReportTypeRequested::ProjectDetails,
    );

    let tracking_result = process_inputs(
        input_path,
        project_details_selected,
        exclude_tags,
        from_date,
        &period,
    )?;

    print_result(period, limit, report_type, &tracking_result, formatter);
    print_warnings(&tracking_result.errors);

    Ok(())
}

fn process_inputs(
    input_path: &Path,
    project_details_selected: Option<String>,
    exclude_tags: Vec<String>,
    from_date: Option<StartDate>,
    period: &Option<PeriodRequested>,
) -> Result<domain::TimeTrackingResult, ParseError> {
    let filter = create_filter(&project_details_selected, &exclude_tags, from_date, period);
    let tracking_result = parsing::process_input(input_path, &filter)?;
    Ok(tracking_result)
}

fn print_result(
    period: Option<PeriodRequested>,
    limit: Option<OutputLimit>,
    report_type: ReportTypeRequested,
    tracking_result: &domain::TimeTrackingResult,
    formatter: Box<dyn Formatter>,
) {
    let period_description = period.map(|p| p.period_description());
    println!("{}", format_header(period_description.as_ref()));

    if let Some(ref time_report) = tracking_result.time_entries {
        let tracked_interval = time_report.period.clone();
        println!("{}", &format_interval(&tracked_interval));

        let report = match report_type {
            ReportTypeRequested::Overview => Report::overview(time_report, limit),
            ReportTypeRequested::ProjectDetails(project) => {
                Report::project_details(&time_report, &project)
            }
        };

        println!("{}", formatter.format(&report));
    } else {
        println!("No data found.");
    }
}

fn print_warnings(parse_errors: &Vec<ParseError>) {
    parse_errors
        .iter()
        .for_each(|error| println!("Warning: {error}"));
}

fn format_header(period_description: Option<&RangeDescription>) -> String {
    let mut result = String::new();

    result.push_str("Time tracking report for ");
    let period_description_str = period_description
        .map(format_period_description)
        .unwrap_or_default();
    result.push_str(period_description_str.as_str());
    result
}

fn create_filter(
    main_context_requested: &Option<String>,
    exclude_tags: &Vec<String>,
    from_date: Option<StartDate>,
    period: &Option<PeriodRequested>,
) -> Option<Filter> {
    let project_filter = main_context_requested.clone().map(Filter::MainContext);
    let from_date_filter = from_date.map(|date| Filter::DateRange(DateRange::new_from_date(date)));
    let period_filter = period
        .clone()
        .map(|period| Filter::DateRange(period.date_range()));

    let exclude_tag_filter = Filter::ExcludeTags(exclude_tags.clone());

    project_filter
        .into_iter()
        .chain(from_date_filter)
        .chain(period_filter)
        .chain(Some(exclude_tag_filter))
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
