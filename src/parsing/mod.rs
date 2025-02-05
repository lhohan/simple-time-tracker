mod filter;
mod parser;
mod processor;

use std::path::Path;

use crate::parsing::processor::FileProcessor;
pub use filter::DateRange;
pub use filter::Filter;
use processor::InputProcessor;

use crate::domain::ParseError;
use crate::domain::ParseResult;
use crate::domain::TimeEntry;
use crate::domain::TrackedTime;
use std::borrow::BorrowMut;

pub fn process_input(
    path: &Path,
    filter: &Option<Filter>,
) -> Result<Option<(TrackedTime, Vec<ParseError>)>, ParseError> {
    let mut combined_result: Option<ParseResult> = None;

    let processor = InputProcessor::from_path(path);

    processor.process(path, |input| {
        if let Some(result) = parser::parse_content(input.content(), filter, input.file_name()) {
            let current = combined_result.borrow_mut();
            *current = Some(match &*current {
                None => result,
                Some(existing) => merge_results(existing, &result),
            });
        }
        Ok(())
    })?;

    Ok(combined_result.map(|parse_result| {
        let entries: Vec<TimeEntry> = parse_result
            .entries_by_date()
            .values()
            .flat_map(|v| v.iter().cloned())
            .collect();
        let errors = parse_result.errors();

        let time_report = TrackedTime::new(
            entries,
            parse_result.start_date(),
            parse_result.end_date(),
            parse_result.days(),
        );

        (time_report, errors)
    }))
}

fn merge_results(first: &ParseResult, second: &ParseResult) -> ParseResult {
    first.merge(second)
}
