pub mod filter;
mod header_parser;
mod line_parser;
mod model;
mod parser;
mod processor;
mod time_parser;

#[cfg(test)]
pub mod test_helpers;

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

    let mut parse_result = ParseResult::errors_only(vec![]);
    processor.process(path, |input| {
        let result = parser::parse_content(input.content(), filter, input.file_name());
        parse_result = parse_result.merge(&result);
        Ok(())
    })?;

    Ok(TimeTrackingResult {
        time_entries: parse_result
            .entries_by_date()
            .filter(|entries| !entries.is_empty())
            .map(|entries| {
                let entries_vec = entries.values().flat_map(|v| v.iter().cloned()).collect();
                let start = StartDate(*entries.keys().min().unwrap());
                let end = EndDate(*entries.keys().max().unwrap());
                TrackedTime::new(entries_vec, start, end, parse_result.days())
            }),
        errors: parse_result.errors(),
    })
}
