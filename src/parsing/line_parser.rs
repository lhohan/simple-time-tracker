use super::time_parser::parse_time;
use crate::domain::{tags::Tag, ParseError, TimeEntry};
use std::collections::VecDeque;

pub(crate) fn parse_entry(line: &str) -> Result<Option<TimeEntry>, ParseError> {
    let line = EntryLine::parse(line);
    match line {
        Some(line) => parse_entry_line(line).map(Some),
        None => Ok(None),
    }
}

struct EntryLine<'a>(pub(crate) &'a str);

impl EntryLine<'_> {
    pub(crate) fn parse(line: &str) -> Option<EntryLine> {
        if EntryLine::is_line_entry(line) {
            Some(EntryLine(line))
        } else {
            None
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
            Ok(LinePart::Tag(project_found)) => {
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
    let projects: Vec<Tag> = projects.into();

    Ok(TimeEntry::new(projects, minutes, description))
}

enum LinePart<'a> {
    Time(u32),
    Tag(Tag),
    DescriptionPart(&'a str),
}

fn parse_part(part: &str) -> Result<LinePart, ParseError> {
    if part.starts_with('#') {
        let raw_tag = part
            .strip_prefix("#")
            .expect("project should have had '#' prefix");
        let tag = Tag::from_raw(raw_tag);
        Ok(LinePart::Tag(tag))
    } else {
        match parse_time(part) {
            Ok(Some(minutes)) => Ok(LinePart::Time(minutes)),
            Ok(None) => Ok(LinePart::DescriptionPart(part)),
            Err(err) => Err(err),
        }
    }
}
