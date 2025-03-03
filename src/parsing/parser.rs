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
    use crate::parsing::test_helpers::LineSpec;

    // Module to test parsing of a plain string into a TimeEntry.
    mod line_entry_parsing {
        use crate::domain::ParseError;
        use crate::parsing::parser::tests::LineSpec;
        use rstest::rstest;

        #[test]
        fn parse_simple_complete_line() {
            LineSpec::given_line("- #project-alpha 10m Task A")
                .when_parsed()
                .expect_valid()
                .expect_minutes(10)
                .expect_main_context("project-alpha")
                .expect_description("Task A");
        }

        #[test]
        fn parse_task_description_is_optional() {
            LineSpec::given_line("- #project-alpha 20m")
                .when_parsed()
                .expect_valid()
                .expect_no_description();
        }

        #[test]
        fn parse_simple_minutes() {
            LineSpec::given_line("- #context 10m")
                .when_parsed()
                .expect_valid()
                .expect_minutes(10);
        }

        #[test]
        fn parse_simple_hours() {
            LineSpec::given_line("- #context 2h")
                .when_parsed()
                .expect_valid()
                .expect_minutes(2 * 60);
        }

        #[test]
        fn parse_pomodoros() {
            LineSpec::given_line("- #context 2p")
                .when_parsed()
                .expect_valid()
                .expect_minutes(2 * 30);
        }

        #[test]
        fn parse_multiple_time_entries() {
            LineSpec::given_line("- #context 1h 10m 1p")
                .when_parsed()
                .expect_valid()
                .expect_minutes(60 + 10 + 30);
        }

        #[rstest]
        fn parse_invalid_line_format(
            #[values("- hash (#) not in start of line", "# dash (-) not in start of line")]
            line: &str,
        ) {
            LineSpec::given_line(line)
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidLineFormat(line.to_string()));
        }

        #[test]
        fn parse_invalid_time() {
            LineSpec::given_line("- #context 100000000000000000000h")
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidTime(
                    "100000000000000000000h".to_string(),
                ));
        }

        #[rstest]
        fn parse_maybe_time(#[values('h', 'm', 'p')] supported_time_unit: char) {
            let input = format!("- #context x{}", supported_time_unit);

            LineSpec::given_line(&input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(input.to_string()));
        }

        #[test]
        fn parse_time_missing() {
            LineSpec::given_line("- #context only description")
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(
                    "- #context only description".to_string(),
                ));
        }

        mod tag_parsing {
            use super::*;

            #[test]
            fn parse_line_with_project_prefix_tag() {
                LineSpec::given_line("- #prj-alpha 1h Task A")
                    .when_parsed()
                    .expect_valid()
                    .expect_main_context("prj-alpha");
            }

            #[test]
            fn parse_line_with_project_and_tags() {
                LineSpec::given_line("- #tag1 #prj-alpha 1h Task A")
                    .when_parsed()
                    .expect_valid()
                    .expect_main_context("prj-alpha");
            }

            #[test]
            fn parse_line_with_only_context_tags() {
                LineSpec::given_line("- #tag1 #tag2 #tag3 1h Task A")
                    .when_parsed()
                    .expect_valid()
                    .expect_main_context("tag1"); // First tag should be used when no project tags
            }
        }
    }

    mod parser_error_handling {
        use crate::domain::ParseError;
        #[test]
        fn error_messages() {
            assert_eq!(
                ParseError::InvalidTime("Xh".to_string()).to_string(),
                "invalid time format: Xh"
            );
        }

        #[test]
        fn error_conversion() {
            let err = ParseError::InvalidTime("a".to_string());
            let _: Box<dyn std::error::Error> = Box::new(err); // Should compile
        }
    }

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
