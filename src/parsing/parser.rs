use crate::parsing::filter::Filter;
use crate::parsing::model::ParseResult;

use crate::domain::dates::EntryDate;
use crate::domain::{Location, ParseError};

use super::{LineType, ParseState, ParsedLine};

pub fn parse_content(content: &str, filter: &Option<Filter>, file_name: &str) -> ParseResult {
    let final_state = content
        .lines()
        .enumerate()
        .map(|(line_number, line)| ParsedLine {
            content: line.trim(),
            line_number: line_number + 1,
        })
        .fold(ParseState::default(), |state, line| {
            process_line(&line, &state, filter, file_name)
        });

    if final_state.entries.is_empty() {
        ParseResult::errors_only(final_state.errors)
    } else {
        ParseResult::new(final_state.entries, final_state.errors)
    }
}

fn process_line(
    line: &ParsedLine,
    state: &ParseState,
    filter: &Option<Filter>,
    file_name: &str,
) -> ParseState {
    match LineType::parse(line.content, state.in_time_tracking_section()) {
        Ok(LineType::Header(maybe_date)) => ParseState {
            current_date: maybe_date,
            ..state.clone()
        },
        Ok(LineType::Entry(entry)) if state.in_time_tracking_section() => {
            let mut new_state = state.clone();
            if let Some(date) = state.current_date {
                match filter {
                    None => new_state.entries.entry(date).or_default().push(entry),
                    Some(filter) if filter.matches(&entry, &EntryDate(date)) => {
                        new_state.entries.entry(date).or_default().push(entry);
                    }
                    _ => {}
                }
            }
            new_state
        }
        Err(error) => ParseState {
            errors: {
                let mut errors = state.errors.clone();
                errors.push(ParseError::Located {
                    error: Box::new(error),
                    location: Location {
                        file: file_name.to_string(),
                        line: line.line_number,
                    },
                });
                errors
            },
            ..state.clone()
        },
        _ => state.clone(),
    }
}

#[cfg(test)]
mod tests {
    mod section_detection {
        use crate::parsing::LineType;
        use rstest::rstest;

        #[rstest]
        fn valid_date_header(
            #[values(
                "# TT 2020-01-01",
                "## TT 2020-01-01",
                "### TT 2020-01-01",
                "############### TT 2020-01-01"
            )]
            input: &str,
        ) {
            assert!(is_date_header(input));
        }

        #[rstest]
        fn detect_date_header(
            #[values(
            "- #context 1h",
            "## Something else",
            "TT 2020-01-01", // No header markers
            "# 2020-01-01", // No TT markers
            "2020-01-01", // Only valid date
            "#TT 2020-01-01", // No space after #
            )]
            input: &str,
        ) {
            assert!(!is_date_header(input));
        }

        fn is_date_header(line: &str) -> bool {
            matches!(LineType::parse(line, true), Ok(LineType::Header(Some(_))))
        }
    }
}
