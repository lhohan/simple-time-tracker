mod filter;
mod parser;
mod processor;

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
use std::borrow::BorrowMut;

pub fn process_input(
    path: &Path,
    filter: &Option<Filter>,
) -> Result<TimeTrackingResult, ParseError> {
    let mut combined_result: Option<ParseResult> = None;

    let processor = InputProcessor::from_path(path);

    processor.process(path, |input| {
        let result = parser::parse_content(input.content(), filter, input.file_name());
        let current = combined_result.borrow_mut();
        *current = Some(match &*current {
            None => result,
            Some(existing) => merge_results(existing, &result),
        });

        Ok(())
    })?;

    let parse_result = combined_result.unwrap_or_else(|| ParseResult::errors_only(vec![]));

    Ok(TimeTrackingResult {
        time_entries: parse_result
            .entries_by_date()
            .filter(|entries| !entries.is_empty())
            .map(|entries| {
                let entries_vec = entries.values().flat_map(|v| v.iter().cloned()).collect();
                let start = StartDate(*entries.keys().min().unwrap()); // Safe because entries not empty
                let end = EndDate(*entries.keys().max().unwrap());
                TrackedTime::new(entries_vec, start, end, parse_result.days())
            }),
        errors: parse_result.errors(),
    })
}

fn merge_results(first: &ParseResult, second: &ParseResult) -> ParseResult {
    first.merge(second)
}
