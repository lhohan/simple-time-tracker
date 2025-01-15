use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

pub fn run(input_path: &Path) -> Result<(), ParseError> {
    let entries = get_entries(input_path)?;
    for entry in entries {
        println!("{}", entry.display());
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    InvalidTime(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "invalid line format"),
            ParseError::InvalidTime(line) => write!(f, "invalid time format: '{}'", line),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq)]
struct TimeEntry {
    project: String,
    minutes: u32,
}

impl TimeEntry {
    fn display(&self) -> String {
        format!("{}: {} minutes", self.project, self.minutes)
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

        let parse_time = |time: &str| -> Result<Option<u32>, ParseError> {
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
        };

        let total_minutes = parts[1..].iter().try_fold(0u32, |acc, &part| {
            parse_time(part).map(|maybe_minutes| acc + maybe_minutes.unwrap_or(0))
        })?;

        if total_minutes > 0 {
            Ok(TimeEntry {
                project,
                minutes: total_minutes,
            })
        } else {
            Err(ParseError::InvalidFormat)
        }
    }
}

fn get_entries(path: &Path) -> Result<Vec<TimeEntry>, ParseError> {
    let content = read_to_string(path).map_err(|_| ParseError::InvalidFormat)?;

    let mut entries = Vec::new();
    for line in content.lines() {
        if line.starts_with("- #") {
            entries.push(TimeEntry::parse_line(line)?);
        }
    }

    Ok(entries)
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
        assert_eq!(TimeEntry::parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_simple_hours() {
        let input = "- #reading 2h";
        let expected = TimeEntry {
            project: "reading".to_string(),
            minutes: 120, // 2 hours = 120 minutes
        };
        assert_eq!(TimeEntry::parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_pomodoros() {
        let input = "- #coding 4p";
        let expected = TimeEntry {
            project: "coding".to_string(),
            minutes: 120, // 4 pomodoros = 120 minutes
        };
        assert_eq!(TimeEntry::parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_invalid_line_format() {
        let input = "not starting with dash and hash";
        assert_eq!(TimeEntry::parse_line(input), Err(ParseError::InvalidFormat));
    }

    #[test]
    fn test_parse_invalid_time_format() {
        let input = "- #reading abch";
        assert_eq!(
            TimeEntry::parse_line(input),
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
        assert_eq!(TimeEntry::parse_line(input), Ok(expected));
    }

    #[test]
    fn test_parse_mixed_content() {
        // Valid times with description
        assert_eq!(
            TimeEntry::parse_line("- #sport 1h some description 30m"),
            Ok(TimeEntry {
                project: "sport".to_string(),
                minutes: 90
            })
        );

        // Invalid time format
        assert_eq!(
            TimeEntry::parse_line("- #sport 1h 30invalid_time_unit 30m"),
            Ok(TimeEntry {
                project: "sport".to_string(),
                minutes: 90
            })
        );

        // No valid times
        assert_eq!(
            TimeEntry::parse_line("- #sport only description"),
            Err(ParseError::InvalidFormat)
        );
    }
}
