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
    use super::*;
    use crate::parsing::test_helpers::LineSpec;

    mod line_parsing {
        use crate::domain::ParseError;
        use crate::parsing::parser::tests::LineSpec;
        use rstest::rstest;

        #[test]
        fn test_parse_bug() {
            let input = "- #prj-1 #health 1h Running";

            LineSpec::new(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(60)
                .expect_project("prj-1")
                .expect_description("Running");
        }

        #[test]
        fn test_parse_simple_complete_line() {
            let input = "- #my_project 20m Worked on Task ...";

            LineSpec::new(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(20)
                .expect_project("my_project")
                .expect_description("Worked on Task ...");
        }

        #[test]
        fn test_parse_simple_minutes() {
            let input = "- #my_project 20m";

            LineSpec::new(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(20);
        }

        #[test]
        fn test_parse_simple_hours() {
            let input = "- #my_project 2h";

            LineSpec::new(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(120);
        }

        #[test]
        fn test_parse_pomodoros() {
            let input = "- #my_project 4p";

            LineSpec::new(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(120);
        }

        #[test]
        fn test_parse_multiple_time_entries() {
            let input = "- #sport 1h 30m";

            LineSpec::new(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(90);
        }

        #[rstest]
        fn test_parse_invalid_line_format(
            #[values("- hash (#) not in start of line", "# dash (-)  not in start of line")] input: &str,
        ) {
            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidLineFormat(input.to_string()));
        }

        #[test]
        fn test_parse_invalid_time() {
            let input = "- #reading 100000000000000000000h";

            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidTime(
                    "100000000000000000000h".to_string(),
                ));
        }

        #[test]
        fn test_parse_maybe_time() {
            let input = "- #reading abch";

            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime("- #reading abch".to_string()));
        }

        #[test]
        fn test_parse_time_missing() {
            let input = "- #my-project only description";

            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(
                    "- #my-project only description".to_string(),
                ));
        }
    }

    #[test]
    fn test_error_messages() {
        assert_eq!(
            ParseError::InvalidTime("abch".to_string()).to_string(),
            "invalid time format: abch"
        );
    }

    #[test]
    fn test_error_conversion() {
        let err = ParseError::InvalidTime("abc".to_string());
        let _: Box<dyn std::error::Error> = Box::new(err); // Should compile
    }

    #[test]
    fn test_detect_date_header() {
        assert!(is_date_header("# TT 2025-01-15"));
        assert!(is_date_header("## TT 2025-01-15"));
        assert!(is_date_header("### TT 2025-01-15"));
        assert!(is_date_header("############### TT 2025-01-15"));

        // Negative cases
        assert!(!is_date_header("- #sport 1h"));
        assert!(!is_date_header("## Something else"));
        assert!(!is_date_header("TT 2025-01-15")); // No header markers
        assert!(!is_date_header("#TT 2025-01-15")); // No space after #
    }

    fn is_date_header(line: &str) -> bool {
        matches!(LineType::parse(line, true), Ok(LineType::Header(Some(_))))
    }
}
