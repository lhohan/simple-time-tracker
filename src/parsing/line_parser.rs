use super::time_parser::parse_time;
use crate::domain::{ParseError, TimeEntry};
use std::collections::VecDeque;

pub(crate) fn parse_entry(line: &str) -> Result<TimeEntry, ParseError> {
    let line = EntryLine::parse(line)?;
    parse_entry_line(line)
}

struct EntryLine<'a>(pub(crate) &'a str);

impl EntryLine<'_> {
    pub(crate) fn parse(line: &str) -> Result<EntryLine, ParseError> {
        if EntryLine::is_line_entry(line) {
            Ok(EntryLine(line))
        } else {
            Err(ParseError::InvalidLineFormat(line.to_string()))
        }
    }

    fn is_line_entry(line: &str) -> bool {
        line.starts_with("- #")
    }

    pub(crate) fn get_line(&self) -> &str {
        &self.0
    }

    // Return the actual content of the line, without the prefix that ids the line is an entry line.
    pub(crate) fn entry(&self) -> &str {
        &self.0.strip_prefix("- ").expect("invalid struct state")
    }
}

fn parse_entry_line(line: EntryLine) -> Result<TimeEntry, ParseError> {
    let line_no_prefix = line.entry();
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
        return Err(ParseError::MissingTime(line.get_line().to_string()));
    }

    if projects.is_empty() {
        return Err(ParseError::InvalidLineFormat("Missing project".to_string()));
    }

    let description =
        (!description.is_empty()).then(|| description.into_iter().collect::<Vec<_>>().join(" "));
    let projects: Vec<String> = projects.into();

    Ok(TimeEntry::new(projects, minutes, description))
}

enum LinePart<'a> {
    Time(u32),
    Project(String),
    DescriptionPart(&'a str),
}

fn parse_part(part: &str) -> Result<LinePart, ParseError> {
    if part.starts_with('#') {
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
