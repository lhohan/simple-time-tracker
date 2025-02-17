use super::line_part_parser::{parse_part, LinePart};
use super::model::LineEntry;
use crate::domain::{ParseError, TimeEntry};
use std::collections::VecDeque;

pub(crate) fn parse_entry(line: LineEntry) -> Result<TimeEntry, ParseError> {
    parse_entry_raw(line.get_str())
}
fn parse_entry_raw(line: &str) -> Result<TimeEntry, ParseError> {
    // TODO: This check could be removed because we check this condition before calling this function. TODO: improve by introducing type?
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

    if projects.is_empty() {
        return Err(ParseError::InvalidLineFormat("Missing project".to_string()));
    }

    let description =
        (!description.is_empty()).then(|| description.into_iter().collect::<Vec<_>>().join(" "));
    let projects: Vec<String> = projects.into();

    Ok(TimeEntry::new(projects, minutes, description))
}
