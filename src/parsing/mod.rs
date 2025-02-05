mod filter;
mod model;
mod parser;
mod processor;

pub(crate) use model::{LineType, ParseState, ParsedLine};

use std::path::Path;

use crate::domain::EndDate;
use crate::domain::ParseResult;
use crate::domain::StartDate;
use crate::domain::TimeTrackingResult;
use crate::parsing::processor::FileProcessor;
pub use filter::DateRange;
pub use filter::Filter;
use processor::InputProcessor;

use crate::domain::ParseError;
use crate::domain::TrackedTime;

pub fn process_input(
    path: &Path,
    filter: &Option<Filter>,
) -> Result<TimeTrackingResult, ParseError> {
    let processor = InputProcessor::from_path(path);

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
