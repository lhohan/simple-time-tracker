pub mod filter;
mod header_parser;
mod model;
mod parser;
mod processor;

use model::ParseResult;
pub(crate) use model::{LineType, ParseState, ParsedLine};
use processor::Processor;

use std::path::Path;

use crate::domain::dates::EndDate;
use crate::domain::dates::StartDate;
use crate::domain::TimeTrackingResult;
use crate::parsing::processor::FileProcessor;
pub use filter::Filter;

use crate::domain::ParseError;
use crate::domain::TrackedTime;

pub fn process_input(
    path: &Path,
    filter: &Option<Filter>,
) -> Result<TimeTrackingResult, ParseError> {
    let processor = Processor::from_path(path);
    let parse_result = parse_entries_from_path(path, filter, &processor)?;
    Ok(tracking_result(&parse_result))
}

fn parse_entries_from_path(
    path: &Path,
    filter: &Option<Filter>,
    processor: &Processor,
) -> Result<ParseResult, ParseError> {
    let mut parse_result = ParseResult::errors_only(vec![]);
    processor.process(path, |input| {
        let result = parser::parse_content(input.content(), filter, input.file_name());
        parse_result = parse_result.merge(&result);
        Ok(())
    })?;
    Ok(parse_result)
}

fn entries(
    entries: &std::collections::HashMap<chrono::NaiveDate, Vec<crate::domain::TimeEntry>>,
) -> Vec<crate::domain::TimeEntry> {
    entries.values().flat_map(|v| v.iter().cloned()).collect()
}

fn start_date(
    mapped_entries: &std::collections::HashMap<chrono::NaiveDate, Vec<crate::domain::TimeEntry>>,
) -> StartDate {
    StartDate(*mapped_entries.keys().min().unwrap())
}

fn end_date(
    mapped_entries: &std::collections::HashMap<chrono::NaiveDate, Vec<crate::domain::TimeEntry>>,
) -> EndDate {
    EndDate(*mapped_entries.keys().max().unwrap())
}

fn days(parse_result: &ParseResult) -> u32 {
    parse_result.days()
}

fn tracking_result(parse_result: &ParseResult) -> TimeTrackingResult {
    let time_entries = tracked_time(parse_result);
    let errors = errors(parse_result);
    TimeTrackingResult {
        time_entries,
        errors,
    }
}

fn tracked_time(parse_result: &ParseResult) -> Option<TrackedTime> {
    parse_result
        .entries_by_date()
        .filter(|entries| !entries.is_empty())
        .map(|mapped_entries| {
            TrackedTime::new(
                entries(mapped_entries),
                start_date(mapped_entries),
                end_date(mapped_entries),
                days(parse_result),
            )
        })
}

fn errors(parse_result: &ParseResult) -> Vec<ParseError> {
    parse_result.errors()
}
