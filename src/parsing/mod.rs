use crate::domain::{ParseError, ParseResult, TimeEntry};
use std::{collections::VecDeque, str::FromStr};

pub fn get_entries(content: &str) -> ParseResult {
    get_entries_from_string(content)
}

fn get_entries_from_string(content: &str) -> ParseResult {
    let (entries, errors, _, days): (Vec<TimeEntry>, Vec<ParseError>, bool, u32) =
        content.lines().map(|line| line.trim()).fold(
            (Vec::new(), Vec::new(), false, 0u32), // Add 'in_tt_section' flag
            |(mut entries, mut errors, mut in_tt_section, days), line| {
                if line.starts_with('#') {
                    in_tt_section = is_date_header(line);
                    if in_tt_section {
                        (entries, errors, in_tt_section, days + 1)
                    } else {
                        (entries, errors, in_tt_section, days)
                    }
                } else if in_tt_section && line.starts_with("- #") {
                    match parse_line(line) {
                        Ok(entry) => (
                            {
                                entries.push(entry);
                                entries
                            },
                            errors,
                            in_tt_section,
                            days,
                        ),
                        Err(e) => (
                            entries,
                            {
                                errors.push(e);
                                errors
                            },
                            in_tt_section,
                            days,
                        ),
                    }
                } else {
                    (entries, errors, in_tt_section, days)
                }
            },
        );

    ParseResult {
        entries,
        errors,
        days,
    }
}

// struct LineEntry<'a>(&'a str);
// fn parse_line1(line: LineEntry) -> Result<TimeEntry, ParseError> {
// parse_line(line.0)
// }

fn parse_line(line: &str) -> Result<TimeEntry, ParseError> {
    if !line.starts_with("- #") {
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
    let project = projects
        .pop_front() // we ignore all projects except first (for now)
        .ok_or(ParseError::InvalidLineFormat("Missing project".to_string()))?
        .to_string();
    let description =
        (!description.is_empty()).then(|| description.into_iter().collect::<Vec<_>>().join(" "));
    Ok(TimeEntry::new(project, minutes, description))
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

fn is_date_header(line: &str) -> bool {
    let mut words = line.trim().split_whitespace();

    matches!(words.next(), Some(first) if first.starts_with('#'))
        && matches!(words.next(), Some("TT"))
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
            assert_eq!(self.project, expected_project.to_string());
            self
        }
        fn expect_description(self, expected_description: &str) -> TimeEntry {
            assert_eq!(self.description, Some(expected_description.to_string()));
            self
        }
    }
}
