use crate::domain::reporting::{DetailReport, OverviewReport};
use crate::domain::tags::Tag;

pub enum FormatableReport<'a> {
    TasksReport(&'a DetailReport),
    OverviewReport(&'a OverviewReport),
}

#[derive(Debug)]
pub enum ReportTypeRequested {
    Overview,
    ProjectDetails(Vec<Tag>),
}
