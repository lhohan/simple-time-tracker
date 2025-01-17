use crate::domain::{ParseError, TimeEntry};

pub fn get_entries(content: &str) -> Result<(Vec<TimeEntry>, u32), ParseError> {
    get_entries_from_string(content)
}

fn get_entries_from_string(content: &str) -> Result<(Vec<TimeEntry>, u32), ParseError> {
    let mut in_tt_section = false;
    let mut days = 0u32;

    let entries = content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with('#') {
                let is_tt = is_date_header(line);
                if is_tt {
                    days += 1;
                }
                in_tt_section = is_tt;
                None
            } else if in_tt_section && line.starts_with("- #") {
                parse_line(line).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok((entries, days))
}

fn parse_line(line: &str) -> Result<TimeEntry, ParseError> {
    let line = line.strip_prefix("- #").ok_or(ParseError::InvalidFormat)?;

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidFormat);
    }

    let (project, parts) = parts.split_first().ok_or(ParseError::InvalidFormat)?;

    let (minutes, description) = parts.iter().fold(
        (0, Vec::new()),
        |(minutes, mut desc), &part| match parse_time(part) {
            Ok(Some(time)) => (minutes + time, desc),
            _ => {
                desc.push(part);
                (minutes, desc)
            }
        },
    );

    if minutes == 0 {
        return Err(ParseError::InvalidFormat);
    }

    Ok(TimeEntry::new(
        project.to_string(),
        minutes,
        (!description.is_empty()).then(|| description.join(" ")),
    ))
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

fn is_date_header(line: &str) -> bool {
    let mut words = line.trim().split_whitespace();

    matches!(words.next(), Some(first) if first.starts_with('#'))
        && matches!(words.next(), Some("TT"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_simple_minutes() {
    // let input = "- #journaling 20m";
    // let expected = TimeEntry {
    // project: "journaling".to_string(),
    // minutes: 20,
    // None,
    // };
    // assert_eq!(parse_line(input), Ok(expected));
    // }

    // #[test]
    // fn test_parse_simple_hours() {
    // let input = "- #reading 2h";
    // let expected = TimeEntry {
    // project: "reading".to_string(),
    // minutes: 120, // 2 hours = 120 minutes
    // None,
    // };
    // assert_eq!(parse_line(input), Ok(expected));
    // }

    // #[test]
    // fn test_parse_pomodoros() {
    // let input = "- #coding 4p";
    // let expected = TimeEntry {
    // project: "coding".to_string(),
    // minutes: 120, // 4 pomodoros = 120 minutes
    // None,
    // };
    // assert_eq!(parse_line(input), Ok(expected));
    // }

    #[test]
    fn test_parse_invalid_line_format() {
        let input = "not starting with dash and hash";
        assert_eq!(parse_line(input), Err(ParseError::InvalidFormat));
    }

    // #[test]
    // fn test_parse_invalid_time_format() {
    // let input = "- #reading abch";
    // assert_eq!(
    // parse_line(input),
    // Err(ParseError::InvalidTime("abch".to_string()))
    // );
    // }

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

    // #[test]
    // fn test_parse_multiple_times() {
    // let input = "- #sport 1h 30m";
    // let expected = TimeEntry {
    // project: "sport".to_string(),
    // minutes: 90, // 1h (60m) + 30m = 90m
    // None,
    // };
    // assert_eq!(parse_line(input), Ok(expected));
    // }

    #[test]
    fn test_parse_invalid_content() {
        // No valid times
        assert_eq!(
            parse_line("- #sport only description"),
            Err(ParseError::InvalidFormat)
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
