use crate::domain::{EntryDate, Location, ParseError, ParseResult, TimeEntry};
use chrono::NaiveDate;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

mod filter;

pub use filter::DateRange;
pub use filter::Filter;

#[derive(Default)]
struct ParseState {
    entries: HashMap<NaiveDate, Vec<TimeEntry>>,
    current_date: Option<NaiveDate>,
    errors: Vec<ParseError>,
}

// design decision: When no entries are found, no ParseResult can exist.
pub fn get_entries(content: &str, filter: &Option<Filter>) -> Option<ParseResult> {
    let final_state = content
        .lines()
        .enumerate()
        .map(|(line_number, line)| (line_number + 1, line.trim())) // Make line reporting 1-based instead 0-based
        .fold(ParseState::default(), |state, (line_number, line)| {
            match (line, state.current_date) {
                (line, _) if line.starts_with('#') => ParseState {
                    current_date: extract_date(line).ok(),
                    ..state
                },
                (line, Some(date)) if line.starts_with("- #") => match parse_line(line) {
                    Ok(entry) => {
                        let mut entries = state.entries;
                        match filter {
                            None => entries.entry(date).or_default().push(entry),
                            Some(filter) => {
                                if filter.matches(&entry, &EntryDate(date)) {
                                    entries.entry(date).or_default().push(entry);
                                }
                            }
                        }
                        ParseState { entries, ..state }
                    }
                    Err(e) => {
                        let mut errors = state.errors;
                        errors.push(ParseError::Located {
                            error: Box::new(e),
                            location: Location {
                                file: String::new(), // Will be filled in later in lib.rs
                                line: line_number,
                            },
                        });
                        ParseState { errors, ..state }
                    }
                },
                _ => state,
            }
        });

    if final_state.entries.is_empty() {
        None
    } else {
        Some(ParseResult::new(final_state.entries, final_state.errors))
    }
}

// struct LineEntry<'a>(&'a str);
// fn parse_line1(line: LineEntry) -> Result<TimeEntry, ParseError> {
// parse_line(line.0)
// }

fn parse_line(line: &str) -> Result<TimeEntry, ParseError> {
    if !line.starts_with("- #") {
        // This check could be removed because we check this condition before calling this function. TODO: improve by introducing type?
        return Err(ParseError::InvalidLineFormat(line.to_string()));
    }
    let line_no_prefix = line
        .strip_prefix("- ")
        .ok_or(ParseError::InvalidLineFormat(line.to_string()))?;
    let parts = line_no_prefix.split_whitespace();

    let mut projects = VecDeque::new();
    let mut minutes = 0;
    let mut description = Vec::new();
    let mut time_found = false;

    for part in parts {
        match parse_part(part) {
            Ok(LinePart::Time(time)) => {
                minutes += time;
                time_found = true;
            }
            Ok(LinePart::Project(project_found)) => {
                projects.push_back(project_found);
            }
            Ok(LinePart::DescriptionPart(desc)) => description.push(desc),
            Err(err) => return Err(err),
        }
    }

    if !time_found {
        return Err(ParseError::MissingTime(line.to_string()));
    }

    // check if there is at least one project:
    if projects.is_empty() {
        return Err(ParseError::InvalidLineFormat("Missing project".to_string()));
    }

    let description =
        (!description.is_empty()).then(|| description.into_iter().collect::<Vec<_>>().join(" "));
    let projects: Vec<String> = projects.into();

    Ok(TimeEntry::new(projects, minutes, description))
}

fn parse_part(part: &str) -> Result<LinePart, ParseError> {
    if part.starts_with("#") {
        let project = LinePart::Project(
            part.strip_prefix("#")
                .expect("project should have had '#' prefix")
                .to_string(),
        );
        Ok(project)
    } else {
        match parse_time(part) {
            Ok(Some(minutes)) => Ok(LinePart::Time(minutes)),
            Ok(None) => Ok(LinePart::DescriptionPart(part)),
            Err(err) => Err(err),
        }
    }
}

enum LinePart<'a> {
    Time(u32),
    Project(String),
    DescriptionPart(&'a str),
}

fn parse_time(time: &str) -> Result<Option<u32>, ParseError> {
    let (value, multiplier) = match time.chars().last() {
        Some('m') => (time.trim_end_matches('m'), 1),
        Some('h') => (time.trim_end_matches('h'), 60),
        Some('p') => (time.trim_end_matches('p'), 30),
        _ => return Ok(None),
    };

    match u32::from_str(value) {
        Ok(val) => Ok(Some(val * multiplier)),
        Err(e) => match e.kind() {
            std::num::IntErrorKind::InvalidDigit => Ok(None),
            _ => Err(ParseError::InvalidTime(time.to_string())),
        },
    }
}

fn maybe_date_from_header(line: &str) -> Option<&str> {
    let mut words = line.trim().split_whitespace();

    if matches!(words.next(), Some(first) if first.starts_with('#'))
        && matches!(words.next(), Some("TT"))
    {
        words.next()
    } else {
        None
    }
}

fn extract_date(line: &str) -> Result<NaiveDate, ParseError> {
    let date_str = maybe_date_from_header(line).ok_or(ParseError::InvalidDate(line.to_string()))?;
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| ParseError::InvalidDate(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod line_parsing {
        use crate::domain::ParseError;
        use crate::parsing::tests::LineSpec;
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
                .expect_invalid_with(ParseError::InvalidLineFormat(input.to_string()));
        }

        #[test]
        fn test_parse_invalid_time() {
            let input = "- #reading 100000000000000000000h";

            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(ParseError::InvalidTime(
                    "100000000000000000000h".to_string(),
                ));
        }

        #[test]
        fn test_parse_maybe_time() {
            let input = "- #reading abch";

            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(ParseError::MissingTime("- #reading abch".to_string()));
        }

        #[test]
        fn test_parse_time_missing() {
            let input = "- #my-project only description";

            LineSpec::new(input)
                .when_parsed()
                .expect_invalid_with(ParseError::MissingTime(
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
        extract_date(line).is_ok()
    }

    struct LineSpec {
        line: String,
    }

    struct LineParsingResult {
        entry: Result<TimeEntry, ParseError>,
    }

    impl LineSpec {
        fn new(line: &str) -> Self {
            LineSpec {
                line: line.to_string(),
            }
        }

        fn when_parsed(self) -> LineParsingResult {
            let obtained = parse_line(&self.line);
            LineParsingResult { entry: obtained }
        }
    }

    impl LineParsingResult {
        fn expect_valid(self) -> TimeEntry {
            self.entry.expect("Expected time entry but was error")
        }

        fn expect_invalid_with(self, expected_error: ParseError) -> () {
            let error = self.entry.expect_err("Expected error but was valid");
            assert_eq!(error, expected_error);
        }
    }

    impl TimeEntry {
        fn expect_minutes(self, expected_minutes: u32) -> TimeEntry {
            assert_eq!(self.minutes, expected_minutes);
            self
        }
        fn expect_project(self, expected_project: &str) -> TimeEntry {
            assert_eq!(*self.main_project(), expected_project.to_string());
            self
        }
        fn expect_description(self, expected_description: &str) -> TimeEntry {
            assert_eq!(self.description, Some(expected_description.to_string()));
            self
        }
    }
}
