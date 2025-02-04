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
use crate::domain::ReportType;
use crate::domain::TimeEntry;
use crate::parsing;
use crate::reporting::Report;
use std::borrow::BorrowMut;

pub fn process_input(
    path: &Path,
    filter: &Option<Filter>,
    report_type: &ReportType,
) -> Result<Option<(Report, Vec<ParseError>)>, ParseError> {
    let mut combined_result: Option<ParseResult> = None;

    let processor = InputProcessor::from_path(path);

    processor.process(path, |content, file_name| {
        if let Some(result) = parser::parse_content(content, filter, file_name) {
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

        let report = match report_type {
            ReportType::ProjectDetails(project) => Report::new_project_detail(
                entries,
                project.clone(),
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            ),
            ReportType::Projects => Report::new_overview(
                entries,
                parse_result.start_date(),
                parse_result.end_date(),
                parse_result.days(),
            ),
        };

        (report, errors)
    }))
}

fn merge_results(first: &ParseResult, second: &ParseResult) -> ParseResult {
    first.merge(second)
}
