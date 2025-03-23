pub mod cli;
mod parsing;
mod reporting;

pub mod domain;

use domain::reports::OutputLimit;
use domain::tags::Tag;
use domain::tags::TagFilter;
use reporting::Report;

use crate::domain::ParseError;
use crate::domain::PeriodRequested;
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
    include_details: bool,
    tag_filter: Option<TagFilter>,
    exclude_tags: Vec<String>,
    period: Option<PeriodRequested>,
    limit: Option<OutputLimit>,
    formatter: Box<dyn Formatter>,
) -> Result<(), ParseError> {
    let tags_details_requested = tag_filter
        .clone()
        .and_then(|filter| filter.tags().first().map(|tag| tag.clone()));

    let tracking_result = process_inputs(input_path, tag_filter, exclude_tags, &period)?;

    print_result(
        period,
        limit,
        include_details,
        &tags_details_requested,
        &tracking_result,
        formatter,
    );
    print_warnings(&tracking_result.errors);

    Ok(())
}

fn process_inputs(
    input_path: &Path,
    tags_filter: Option<TagFilter>,
    exclude_tags: Vec<String>,
    period: &Option<PeriodRequested>,
) -> Result<domain::TimeTrackingResult, ParseError> {
    let filter = create_filter(&tags_filter, &exclude_tags, period);
    let tracking_result = parsing::process_input(input_path, &filter)?;
    Ok(tracking_result)
}

fn print_result(
    period: Option<PeriodRequested>,
    limit: Option<OutputLimit>,
    include_details: bool,
    project: &Option<Tag>,
    tracking_result: &domain::TimeTrackingResult,
    formatter: Box<dyn Formatter>,
) {
    if let Some(ref time_report) = tracking_result.time_entries {
        let report_type = if include_details {
            ReportTypeRequested::ProjectDetails(vec![project
                .clone()
                .expect("tags filter does not contain project")])
        } else {
            ReportTypeRequested::Overview
        };

        let report = match report_type {
            ReportTypeRequested::Overview => Report::overview(time_report, limit, &period),
            ReportTypeRequested::ProjectDetails(tags) => {
                Report::project_details(&time_report, &tags.first().unwrap().clone())
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

fn create_filter(
    tags_filter: &Option<TagFilter>,
    exclude_tags: &Vec<String>,
    period: &Option<PeriodRequested>,
) -> Option<Filter> {
    let period_filter = period
        .clone()
        .map(|period| Filter::DateRange(period.date_range()));
    let tags_filter = tags_filter
        .as_ref()
        .map(|filter| Filter::Tags(filter.filter_tags()));
    let exclude_tag_filter = Filter::ExcludeTags(exclude_tags.clone());

    tags_filter
        .into_iter()
        .chain(period_filter)
        .chain(Some(exclude_tag_filter))
        .reduce(Filter::combine)
}
