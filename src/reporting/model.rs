use crate::domain::reporting::{DetailReport, OverviewReport};

pub enum FormatableReport<'a> {
    TasksReport(&'a DetailReport),
    OverviewReport(&'a OverviewReport),
}
