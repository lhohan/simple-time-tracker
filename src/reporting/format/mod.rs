//! Module for report formatting in different output formats

mod markdown;
mod text;

use crate::reporting::format::markdown::MarkdownFormatter;
use crate::reporting::format::text::TextFormatter;
use crate::reporting::model::Report;
use std::fmt;

use super::model::FormatableReport;

fn create_formatter(format_type: FormatType) -> Box<dyn Formatter> {
    match format_type {
        FormatType::Text => Box::new(TextFormatter),
        FormatType::Markdown => Box::new(MarkdownFormatter),
    }
}

pub trait Formatter {
    fn format_legacy(&self, report: &Report) -> String {
        self.format(&FormatableReport::LegacyReport(report))
    }
    fn format(&self, report: &FormatableReport) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormatType {
    Text,
    Markdown,
}

impl dyn Formatter {
    pub fn from_str(s: &Option<String>) -> Box<dyn Formatter> {
        match s {
            Some(s) => {
                let format_type = FormatType::from_str(s);
                create_formatter(format_type)
            }
            None => Box::new(TextFormatter),
        }
    }
}

impl FormatType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => FormatType::Markdown,
            "text" => FormatType::Text,
            _ => FormatType::Text,
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            <TextFormatter as Formatter>::format_legacy(&TextFormatter, self)
        )
    }
}

pub(crate) fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    format!("{hours:2}h {remaining_minutes:02}m")
}
