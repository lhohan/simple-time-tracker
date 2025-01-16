use crate::domain::{ParseError, TimeEntry};
use crate::utils::Pipe;

pub fn get_entries(content: &str) -> Result<Vec<TimeEntry>, ParseError> {
    let entries = get_entries_from_string(content)?;
    Ok(summarize_entries(&entries))
}

fn get_entries_from_string(content: &str) -> Result<Vec<TimeEntry>, ParseError> {
    let mut in_tt_section = false;

    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with('#') {
                in_tt_section = is_date_header(line);
                None
            } else if in_tt_section && line.starts_with("- #") {
                parse_line(line).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .pipe(|entries| Ok(entries))
}

fn parse_line(line: &str) -> Result<TimeEntry, ParseError> {
    if !line.starts_with("- #") {
        return Err(ParseError::InvalidFormat);
    }

    let parts: Vec<&str> = line.trim_start_matches("- #").split_whitespace().collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidFormat);
    }

    let project = parts[0].to_string();

    let total_minutes = parts[1..].iter().try_fold(0u32, |acc, &part| {
        parse_time(part).map(|maybe_minutes| acc + maybe_minutes.unwrap_or(0))
    })?;

    if total_minutes > 0 {
        Ok(TimeEntry::new(project, total_minutes))
    } else {
        Err(ParseError::InvalidFormat)
    }
}

fn parse_time(time: &str) -> Result<Option<u32>, ParseError> {
    match time {
        t if t.ends_with('m') => t
            .trim_end_matches('m')
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidTime(t.to_string()))
            .map(Some),
        t if t.ends_with('h') => t
            .trim_end_matches('h')
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidTime(t.to_string()))
            .map(|h| Some(h * 60)),
        t if t.ends_with('p') => t
            .trim_end_matches('p')
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidTime(t.to_string()))
            .map(|p| Some(p * 30)),
        _ => Ok(None),
    }
}

fn summarize_entries(entries: &[TimeEntry]) -> Vec<TimeEntry> {
    let mut summary = std::collections::HashMap::new();

    for entry in entries {
        *summary.entry(entry.project.clone()).or_insert(0) += entry.minutes;
    }

    let mut result: Vec<_> = summary
        .into_iter()
        .map(|(project, minutes)| TimeEntry::new(project, minutes))
        .collect();
    result.sort_by(|a, b| a.project.cmp(&b.project));
    result
}

fn is_date_header(line: &str) -> bool {
    let mut words = line.trim().split_whitespace();

    matches!(words.next(), Some(first) if first.starts_with('#'))
        && matches!(words.next(), Some("TT"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_minutes() {
        let input = "- #journaling 20m";
        let expected = TimeEntry {
            project: "journaling".to_string(),
            minutes: 20,
        };
        assert_eq!(parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_simple_hours() {
        let input = "- #reading 2h";
        let expected = TimeEntry {
            project: "reading".to_string(),
            minutes: 120, // 2 hours = 120 minutes
        };
        assert_eq!(parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_pomodoros() {
        let input = "- #coding 4p";
        let expected = TimeEntry {
            project: "coding".to_string(),
            minutes: 120, // 4 pomodoros = 120 minutes
        };
        assert_eq!(parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_invalid_line_format() {
        let input = "not starting with dash and hash";
        assert_eq!(parse_line(input), Err(ParseError::InvalidFormat));
    }

    #[test]
    fn test_parse_invalid_time_format() {
        let input = "- #reading abch";
        assert_eq!(
            parse_line(input),
            Err(ParseError::InvalidTime("abch".to_string()))
        );
    }

    #[test]
    fn test_error_messages() {
        assert_eq!(ParseError::InvalidFormat.to_string(), "invalid line format");
        assert_eq!(
            ParseError::InvalidTime("abch".to_string()).to_string(),
            "invalid time format: 'abch'"
        );
    }

    #[test]
    fn test_error_conversion() {
        let err = ParseError::InvalidTime("abc".to_string());
        let _: Box<dyn std::error::Error> = Box::new(err); // Should compile
    }

    #[test]
    fn test_parse_multiple_times() {
        let input = "- #sport 1h 30m";
        let expected = TimeEntry {
            project: "sport".to_string(),
            minutes: 90, // 1h (60m) + 30m = 90m
        };
        assert_eq!(parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_mixed_content() {
        // Valid times with description
        assert_eq!(
            parse_line("- #sport 1h some description 30m"),
            Ok(TimeEntry {
                project: "sport".to_string(),
                minutes: 90
            })
        );

        // Invalid time format
        assert_eq!(
            parse_line("- #sport 1h 30invalid_time_unit 30m"),
            Ok(TimeEntry {
                project: "sport".to_string(),
                minutes: 90
            })
        );

        // No valid times
        assert_eq!(
            parse_line("- #sport only description"),
            Err(ParseError::InvalidFormat)
        );
    }

    #[test]
    fn test_get_entries_from_string() {
        let input = r#"## TT 2025-01-15
- #sport 1h
- #sport 30m
- #coding 2p"#;

        let entries = get_entries(input).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[0],
            TimeEntry {
                project: "coding".to_string(),
                minutes: 60
            }
        );
        assert_eq!(
            entries[1],
            TimeEntry {
                project: "sport".to_string(),
                minutes: 90
            }
        );
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
        assert!(!is_date_header("TTT 2025-01-15")); // No header markers
        assert!(!is_date_header("#TT 2025-01-15")); // No space after #
    }
}
