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
                    filter.map_or(true, |f| f.matches(&entry, &EntryDate(date)));
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic parser functionality
    #[test]
    fn parser_works_correctly() {
        let test_content = r#"
## TT 2024-01-01
- #dev 30m implement feature A
- #test 15m write tests

## TT 2024-01-02
- #dev 1h fix bug B
"#;

        let result = parse_content(test_content, None, "test.md");

        assert_eq!(result.days(), 2);
        assert_eq!(result.errors().len(), 0);

        if let Some(entries) = result.entries_by_date() {
            assert_eq!(entries.len(), 2);
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
