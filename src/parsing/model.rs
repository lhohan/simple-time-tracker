use crate::domain::{ParseError, TimeEntry};
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub(crate) struct ParseState {
    pub(crate) entries: HashMap<NaiveDate, Vec<TimeEntry>>,
    pub(crate) current_date: Option<NaiveDate>,
    pub(crate) errors: Vec<ParseError>,
}

pub(crate) struct ParsedLine<'a> {
    pub(crate) content: &'a str,
    pub(crate) line_number: usize,
}

pub(crate) enum LineType {
    Header(Option<NaiveDate>),
    Entry(TimeEntry),
    Other,
}
