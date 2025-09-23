pub mod filter;
mod header_parser;
mod model;
mod parser;
mod processor;

use model::ContentParseResults;
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
    filter: Option<&Filter>,
) -> Result<TimeTrackingResult, ParseError> {
    let processor = Processor::from_path(path);
    let parse_result = parse_entries_from_path(path, filter, &processor)?;
    Ok(tracking_result(&parse_result))
}

fn parse_entries_from_path(
    path: &Path,
    filter: Option<&Filter>,
    processor: &Processor,
) -> Result<ContentParseResults, ParseError> {
    let mut parse_result = ContentParseResults::errors_only(vec![]);
    processor.process(path, |input| {
        let result = parser::parse_content(input.content(), filter, input.file_name());
        parse_result = parse_result.merge(&result);
        Ok(())
    })?;
    Ok(parse_result)
}

fn tracking_result(parse_result: &ContentParseResults) -> TimeTrackingResult {
    let time_entries = tracked_time(parse_result);
    let errors = errors(parse_result);
    TimeTrackingResult {
        time_entries,
        errors,
    }
}

fn tracked_time(parse_result: &ContentParseResults) -> Option<TrackedTime> {
    parse_result
        .entries_by_date()
        .filter(|entries| !entries.is_empty()) // no tracked time
        .and_then(|entries| {
            let start = entries.keys().min().map(|&date| StartDate(date))?;
            let end = entries.keys().max().map(|&date| EndDate(date))?;
            let days = parse_result.days();
            let entries = entries.values().flat_map(|v| v.iter().cloned()).collect();
            Some(TrackedTime::new(entries, start, end, days))
        })
}

fn errors(parse_result: &ContentParseResults) -> Vec<ParseError> {
    parse_result.errors()
}
