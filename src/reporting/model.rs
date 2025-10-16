use crate::domain::reporting::{BreakdownReport, DetailReport, OverviewReport};

#[allow(clippy::enum_variant_names)]
pub enum FormatableReport<'a> {
    TasksReport(&'a DetailReport),
    OverviewReport(&'a OverviewReport),
    BreakdownReport(&'a BreakdownReport),
}
