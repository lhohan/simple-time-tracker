use crate::parsing::filter::Filter;
use crate::parsing::model::ContentParseResults;

use crate::domain::dates::EntryDate;
use crate::domain::{Location, ParseError};

use super::{LineType, ParseState, ParsedLine};

#[must_use]
pub fn parse_content(
    content: &str,
    filter: Option<&Filter>,
    file_name: &str,
) -> ContentParseResults {
    let mut state = ParseState::default();

    for (line_number, line) in content.lines().enumerate() {
        let parsed_line = ParsedLine {
            content: line.trim(),
            line_number: line_number + 1,
        };
        process_line_mut(&parsed_line, &mut state, filter, file_name);
    }

    if state.entries.is_empty() {
        ContentParseResults::errors_only(state.errors)
    } else {
        ContentParseResults::new(state.entries, state.errors)
    }
}

fn process_line_mut(
    line: &ParsedLine,
    state: &mut ParseState,
    filter: Option<&Filter>,
    file_name: &str,
) {
    match LineType::parse(line.content, state.in_time_tracking_section()) {
        Ok(LineType::Header(maybe_date)) => {
            state.current_date = maybe_date;
        }
        Ok(LineType::Entry(entry)) if state.in_time_tracking_section() => {
            if let Some(date) = state.current_date {
                let entry_matches_filter =
                    filter.is_none_or(|f| f.matches(&entry, &EntryDate(date)));
                if entry_matches_filter {
                    state.entries.entry(date).or_default().push(entry);
                }
            }
        }
        Err(error) => {
            state.errors.push(ParseError::Located {
                error: Box::new(error),
                location: Location {
                    file: file_name.to_string(),
                    line: line.line_number,
                },
            });
        }
        _ => {}
    }
}
