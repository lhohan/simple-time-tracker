//! Time tracking reporting and breakdown functionality.
//!
//! This module provides data structures and functions for aggregating and reporting
//! time tracking data across different time periods and hierarchies.
//!
//! # Overview
//!
//! The reporting module supports three types of reports:
//!
//! - **Overview Report**: Summary of time by tags and outcomes
//! - **Detail Report**: Task-level breakdown for specific tags
//! - **Breakdown Report**: Hierarchical time aggregation by calendar units
//!
//! # Breakdown Feature
//!
//! The breakdown feature provides hierarchical time aggregation across calendar units:
//!
//! ## Aggregation Hierarchy
//!
//! - **Day**: Flat list of days with entries (no children)
//! - **Week**: ISO weeks containing days as children
//! - **Month**: Calendar months containing ISO weeks as children (not days)
//! - **Year**: Calendar years containing months as children
//!
//! ## Design Decisions
//!
//! ### Month → Week Aggregation
//!
//! Months aggregate to **weeks** rather than days to keep output concise for long
//! periods. This prevents overwhelming output when viewing multi-month ranges.
//! Users can use week breakdown for day-level detail.
//!
//! Example output for month breakdown:
//! ```text
//! 2024-01
//!   2024-W01  10h 30m
//!   2024-W02  15h 45m
//!   2024-W03  12h 15m
//! ```
//!
//! ### ISO Week Standard
//!
//! All week calculations use **ISO 8601 week dates**:
//!
//! - Weeks start on Monday and end on Sunday
//! - Week 1 is the week with the year's first Thursday
//! - Weeks are numbered 01 to 52 (or 53)
//! - Format: `YYYY-W##` (e.g., `2024-W01`)
//!
//! #### Important ISO Week Edge Cases
//!
//! **Week 53 spanning into next year:**
//! ```text
//! 2020-12-28 (Mon) → 2020-W53
//! 2020-12-31 (Thu) → 2020-W53
//! 2021-01-01 (Fri) → 2020-W53  // Calendar year 2021, but ISO week from 2020
//! 2021-01-04 (Mon) → 2021-W01  // First day of 2021's Week 1
//! ```
//!
//! **Week 1 starting in previous year:**
//! ```text
//! 2020-12-28 (Mon) → 2020-W53
//! 2021-01-01 (Fri) → 2020-W53  // Jan 1 belongs to previous year's week
//! 2021-01-04 (Mon) → 2021-W01  // Week 1 of 2021 starts here
//! ```
//!
//! **Weeks spanning month boundaries (~20% of weeks):**
//! ```text
//! 2023-W05:
//!   2023-01-30 (Mon)  // Last day of January
//!   2023-01-31 (Tue)
//!   2023-02-01 (Wed)  // First day of February
//!   ...
//!   2023-02-05 (Sun)
//! ```
//!
//! In month breakdowns, weeks spanning months appear in **both** months with
//! appropriate time attribution.
//!
//! ### Leap Year Handling
//!
//! February 29 is handled correctly in all breakdown levels:
//! - Day breakdowns include `2024-02-29 (Thu)`
//! - Week breakdowns include Feb 29 within the appropriate week
//! - Month/year aggregations include leap day time in totals
//!
//! ### Sparse Data
//!
//! Empty periods are automatically omitted from output:
//! - Days with zero entries don't appear in day breakdowns
//! - Weeks with zero entries don't appear in week/month breakdowns
//! - Months with zero entries don't appear in year breakdowns
//!
//! This keeps output focused on periods with actual tracked time.
//!
//! # Examples
//!
//! ## Creating a Breakdown Report
//!
//! ```ignore
//! use time_tracker::domain::reporting::{BreakdownReport, BreakdownUnit};
//!
//! // From parsed time entries
//! let report = BreakdownReport::from_tracked_time(&tracked_time, BreakdownUnit::Week);
//!
//! // Iterate over top-level groups (weeks)
//! for week in &report.groups {
//!     println!("{}: {}h {}m", week.label, week.minutes / 60, week.minutes % 60);
//!
//!     // Iterate over children (days within week)
//!     for day in &week.children {
//!         println!("  {}: {}h {}m", day.label, day.minutes / 60, day.minutes % 60);
//!     }
//! }
//! ```
//!
//! ## Output Formats
//!
//! ```text
//! # Week Breakdown (week → days)
//! 2024-W09                    6h 00m
//!   2024-02-26 (Mon)          1h 00m
//!   2024-02-29 (Thu)          3h 00m  # Leap day
//!   2024-03-01 (Fri)          2h 00m
//!
//! # Month Breakdown (month → weeks)
//! 2024-01                    38h 15m
//!   2024-W01                 10h 30m
//!   2024-W03                 12h 15m  # W02 omitted (no entries)
//!   2024-W04                 15h 30m
//!
//! # Year Breakdown (year → months)
//! 2024                      245h 30m
//!   2024-01                  38h 15m
//!   2024-02                  42h 00m
//!   2024-03                  35h 45m
//! ```

use std::collections::HashMap;
use std::hash::Hash;

use super::dates::{EndDate, StartDate};
use super::tags::Tag;
use super::{ParseError, PeriodRequested, TimeEntry};
use chrono::NaiveDate;
use chrono::{Datelike, IsoWeek};
use itertools::Itertools;

#[derive(Debug)]
pub struct TimeTrackingResult {
    pub time_entries: Option<TrackedTime>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug)]
pub struct TrackedTime {
    pub entries: Vec<TimeEntry>,
    pub period: TrackingPeriod,
    pub total_minutes: u32,
    pub entries_by_date: std::collections::HashMap<NaiveDate, Vec<TimeEntry>>,
}

impl TrackedTime {
    #[must_use]
    pub fn new(entries: Vec<TimeEntry>, start: StartDate, end: EndDate, days: u32) -> Self {
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            period: TrackingPeriod::new(start, end, days),
            total_minutes,
            entries_by_date: std::collections::HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_entries_by_date(
        entries: Vec<TimeEntry>,
        entries_by_date: std::collections::HashMap<NaiveDate, Vec<TimeEntry>>,
        start: StartDate,
        end: EndDate,
        days: u32,
    ) -> Self {
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            period: TrackingPeriod::new(start, end, days),
            total_minutes,
            entries_by_date,
        }
    }

    #[must_use]
    pub fn tasks_tracked_for(&self, tags: &[Tag]) -> DetailReport {
        let mut per_tag_summaries = Vec::new();

        for tag in tags {
            let tag_summary = self.summarize_tasks_for_context(tag);
            per_tag_summaries.push(tag_summary);
        }

        DetailReport::new(per_tag_summaries, self.period, self.total_minutes)
    }

    fn summarize_tasks_for_context(&self, tag: &Tag) -> TaskSummariesForContext {
        let total_times_for_tasks = self.calculate_totals_for_tasks(tag);

        let total_time_for_tag: u32 = total_times_for_tasks.values().sum();

        let task_summaries = total_times_for_tasks
            .into_iter()
            .map(|(desc, minutes)| TaskSummary::new(desc, minutes, total_time_for_tag));
        let sorted_summaries = TrackedTime::sort_by_time(task_summaries).collect::<Vec<_>>();

        TaskSummariesForContext::new(tag.clone(), sorted_summaries)
    }

    fn calculate_totals_for_tasks(&self, tag: &Tag) -> HashMap<String, u32> {
        sum_time_by_key(
            self.entries.iter().filter(|entry| entry.tags.contains(tag)),
            |entry| {
                Some(
                    entry
                        .description
                        .clone()
                        .unwrap_or_else(|| "<no description>".to_string()),
                )
            },
        )
    }

    fn sort_by_time(
        map: std::iter::Map<
            std::collections::hash_map::IntoIter<String, u32>,
            impl FnMut((String, u32)) -> TaskSummary,
        >,
    ) -> std::vec::IntoIter<TaskSummary> {
        map.sorted_by(|a, b| b.minutes.cmp(&a.minutes))
    }
}

pub struct OverviewReport {
    entries_total_time: Vec<TimeTotal>,
    outcomes_total_time: Vec<TimeTotal>,
    period: TrackingPeriod,
    period_requested: Option<PeriodRequested>,
    total_minutes: u32,
}

impl OverviewReport {
    #[must_use]
    pub fn overview(
        time_report: &TrackedTime,
        limit: Option<&OutputLimit>,
        period_requested: Option<&PeriodRequested>,
    ) -> Self {
        let entries_summed = sum_time_entries(time_report, limit);
        let outcomes_summed = sum_outcomes(time_report);

        OverviewReport {
            entries_total_time: entries_summed,
            outcomes_total_time: outcomes_summed,
            period: time_report.period,
            period_requested: period_requested.cloned(),
            total_minutes: time_report.total_minutes,
        }
    }

    #[must_use]
    pub fn entries_time_totals(&self) -> &Vec<TimeTotal> {
        &self.entries_total_time
    }

    #[must_use]
    pub fn outcome_time_totals(&self) -> &Vec<TimeTotal> {
        &self.outcomes_total_time
    }

    #[must_use]
    pub fn period(&self) -> &TrackingPeriod {
        &self.period
    }

    #[must_use]
    pub fn period_requested(&self) -> &Option<PeriodRequested> {
        &self.period_requested
    }

    #[must_use]
    pub fn total_minutes(&self) -> u32 {
        self.total_minutes
    }
}

fn sum_time_entries(time_report: &TrackedTime, limit: Option<&OutputLimit>) -> Vec<TimeTotal> {
    let summed_entries = sum_entries(&time_report.entries);

    let summed_entries_sorted = summed_entries
        .into_iter()
        .map(|(project, minutes)| TimeTotal::new(project, minutes, time_report.total_minutes))
        .sorted_by(|a, b| {
            b.minutes
                .cmp(&a.minutes)
                .then(a.description.cmp(&b.description))
        });

    match limit {
        Some(OutputLimit::CumulativePercentageThreshold(threshold)) => {
            let total_minutes = f64::from(time_report.total_minutes);
            limit_number_of_entries(total_minutes, summed_entries_sorted, *threshold)
        }
        None => summed_entries_sorted.collect(),
    }
}

fn limit_number_of_entries(
    total_minutes: f64,
    totals: std::vec::IntoIter<TimeTotal>,
    cumulative_percentage_threshold: f64,
) -> Vec<TimeTotal> {
    let mut result = Vec::new();
    let mut cumulative_percentage = 0.0;

    for total in totals {
        let percentage = (f64::from(total.minutes) / total_minutes) * 100.0;
        cumulative_percentage += percentage;
        result.push(total);

        if cumulative_percentage >= cumulative_percentage_threshold {
            break;
        }
    }
    result
}

fn sum_time_by_key<'a, F, K>(
    entries: impl Iterator<Item = &'a TimeEntry>,
    key_extractor: F,
) -> HashMap<K, u32>
where
    F: Fn(&TimeEntry) -> Option<K>,
    K: Eq + Hash,
{
    let mut aggregated = HashMap::new();
    for entry in entries {
        if let Some(key) = key_extractor(entry) {
            *aggregated.entry(key).or_insert(0) += entry.minutes;
        }
    }
    aggregated
}

fn sum_entries(entries: &[TimeEntry]) -> Vec<(String, u32)> {
    sum_time_by_key(entries.iter(), |entry| Some(entry.main_context().clone()))
        .into_iter()
        .collect()
}

fn sum_outcomes(time_report: &TrackedTime) -> Vec<TimeTotal> {
    sum_time_by_key(time_report.entries.iter(), |entry| {
        entry
            .outcome
            .as_ref()
            .map(|outcome| outcome.description().to_string())
    })
    .into_iter()
    .map(|(outcome, duration)| TimeTotal::new(outcome, duration, time_report.total_minutes))
    .collect()
}

#[derive(Debug, Clone)]
pub struct TimeTotal {
    pub(crate) description: String,
    pub(crate) minutes: u32,
    pub(crate) percentage: u32,
}

impl TimeTotal {
    #[must_use]
    pub fn new(description: String, minutes: u32, total_minutes: u32) -> Self {
        Self {
            description,
            minutes,
            percentage: calculate_percentage(minutes, total_minutes),
        }
    }
}

pub struct DetailReport {
    summaries: Vec<TaskSummariesForContext>,
    period: TrackingPeriod,
    total_minutes: u32,
}

impl DetailReport {
    #[must_use]
    pub fn new(
        summaries: Vec<TaskSummariesForContext>,
        period: TrackingPeriod,
        total_minutes: u32,
    ) -> Self {
        Self {
            summaries,
            period,
            total_minutes,
        }
    }

    #[must_use]
    pub fn summaries(&self) -> &Vec<TaskSummariesForContext> {
        &self.summaries
    }

    #[must_use]
    pub fn period(&self) -> &TrackingPeriod {
        &self.period
    }

    #[must_use]
    pub fn total_minutes(&self) -> u32 {
        self.total_minutes
    }
}

#[derive(Debug)]
pub struct TaskSummariesForContext {
    context: Tag,
    entries: Vec<TaskSummary>,
}

impl TaskSummariesForContext {
    fn new(context: Tag, entries: Vec<TaskSummary>) -> Self {
        Self { context, entries }
    }

    #[must_use]
    pub fn context(&self) -> &Tag {
        &self.context
    }

    #[must_use]
    pub fn task_summaries(&self) -> &Vec<TaskSummary> {
        &self.entries
    }

    #[must_use]
    pub fn total_minutes(&self) -> u32 {
        self.entries.iter().map(|entry| entry.minutes).sum()
    }
}

#[derive(Debug)]
pub struct TaskSummary {
    pub(crate) description: String,
    pub(crate) minutes: u32,
    pub(crate) percentage_of_total: u32,
}

impl TaskSummary {
    #[must_use]
    pub fn new(description: String, minutes: u32, total_minutes: u32) -> Self {
        Self {
            description,
            minutes,
            percentage_of_total: calculate_percentage(minutes, total_minutes),
        }
    }
}

fn calculate_percentage(minutes: u32, total_minutes: u32) -> u32 {
    if total_minutes == 0 {
        return 0;
    }

    let percentage = (f64::from(minutes) / f64::from(total_minutes)) * 100.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let result = percentage.round() as u32;
    result
}

#[derive(Debug, Clone)]
pub struct PeriodDescription(String);

impl PeriodDescription {
    #[must_use]
    pub fn day(date: NaiveDate) -> Self {
        let date_str = format_day(date);
        PeriodDescription(date_str)
    }

    #[must_use]
    pub fn from_date(date: NaiveDate) -> Self {
        let date_str = format_from_date(date);
        PeriodDescription(date_str)
    }

    #[must_use]
    pub fn week_of(date: NaiveDate) -> Self {
        let week = date.iso_week();
        let week_str = format_week(week);
        PeriodDescription(week_str)
    }

    #[must_use]
    pub fn month_of(date: NaiveDate) -> Self {
        let month_str = format_month(date);
        PeriodDescription(month_str)
    }

    #[must_use]
    pub fn year_of(date: NaiveDate) -> Self {
        let year_str = format_year(date);
        PeriodDescription(year_str)
    }
}

fn format_day(date: NaiveDate) -> String {
    let date_str = format_yyyy_mm_dd(date);
    format!("of {date_str}")
}

fn format_yyyy_mm_dd(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

fn format_from_date(date: NaiveDate) -> String {
    format!("from {} until today", format_yyyy_mm_dd(date))
}

fn format_week(week: IsoWeek) -> String {
    let week_number = week.week();
    let year = week.year();
    format!("of week {week_number}, {year}")
}

fn format_month(date: NaiveDate) -> String {
    format!("of {}", date.format("%Y-%m"))
}

fn format_year(date: NaiveDate) -> String {
    format!("of {}", date.format("%Y"))
}

impl std::fmt::Display for PeriodDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrackingPeriod {
    pub(crate) start: StartDate,
    pub(crate) end: EndDate,
    pub(crate) days: u32,
}

impl TrackingPeriod {
    #[must_use]
    pub fn new(start: StartDate, end: EndDate, days: u32) -> Self {
        Self { start, end, days }
    }
}

pub enum OutputLimit {
    CumulativePercentageThreshold(f64),
}

/// Calendar unit for hierarchical time breakdown aggregation.
///
/// Determines how time entries are grouped in breakdown reports. Each unit
/// creates a different level of aggregation with specific parent-child relationships.
///
/// # Aggregation Hierarchy
///
/// - `Day`: Flat list, no children
/// - `Week`: ISO weeks → days
/// - `Month`: Calendar months → ISO weeks (not days)
/// - `Year`: Calendar years → months
///
/// # ISO Week Standard
///
/// Week calculations follow ISO 8601:
/// - Weeks start Monday, end Sunday
/// - Week 1 contains the year's first Thursday
/// - Weeks numbered 01-52 (or 53 in long years)
/// - Format: `YYYY-W##`
///
/// # Examples
///
/// ```ignore
/// use time_tracker::domain::reporting::BreakdownUnit;
///
/// let unit = BreakdownUnit::Week;  // Group by ISO weeks, show days within weeks
/// let unit = BreakdownUnit::Month; // Group by months, show weeks within months
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakdownUnit {
    /// Day-level breakdown: flat list of days with time entries.
    ///
    /// No hierarchical structure (children always empty).
    ///
    /// Output format: `2024-02-29 (Thu)`
    Day,

    /// Week-level breakdown: ISO weeks containing days as children.
    ///
    /// Groups time entries by ISO week, with individual days shown within each week.
    /// Handles year boundaries correctly (e.g., 2021-01-01 may be in 2020-W53).
    ///
    /// Output format:
    /// ```text
    /// 2024-W09
    ///   2024-02-26 (Mon)
    ///   2024-02-29 (Thu)
    /// ```
    Week,

    /// Month-level breakdown: calendar months containing ISO weeks as children.
    ///
    /// **Design decision:** Aggregates to weeks, not days, for concise output.
    /// Use `Week` breakdown if day-level detail is needed.
    ///
    /// Output format:
    /// ```text
    /// 2024-01
    ///   2024-W01
    ///   2024-W03
    /// ```
    Month,

    /// Year-level breakdown: calendar years containing months as children.
    ///
    /// Groups by calendar year with month-level children. Does not show weeks or days.
    ///
    /// Output format:
    /// ```text
    /// 2024
    ///   2024-01
    ///   2024-02
    /// ```
    Year,
}

/// A group of time entries aggregated by a calendar unit.
///
/// Forms a hierarchical tree structure for breakdown reports. Each group has:
/// - A human-readable label (e.g., "2024-W09", "2024-01-15 (Mon)")
/// - Total minutes for this group (sum of all time entries)
/// - Optional child groups for hierarchical breakdowns
///
/// # Hierarchy Depth
///
/// - **Day breakdown**: 1 level (days only, no children)
/// - **Week breakdown**: 2 levels (weeks → days)
/// - **Month breakdown**: 2 levels (months → weeks)
/// - **Year breakdown**: 2 levels (years → months)
///
/// # Invariants
///
/// - `minutes` equals sum of children's minutes (if children exist)
/// - Empty groups (minutes = 0) are filtered out
/// - Groups are sorted chronologically
/// - Labels follow consistent format per breakdown unit
///
/// # Examples
///
/// ```ignore
/// // Week group with day children
/// BreakdownGroup {
///     label: "2024-W09".to_string(),
///     minutes: 360,  // 6 hours total
///     children: vec![
///         BreakdownGroup {
///             label: "2024-02-26 (Mon)".to_string(),
///             minutes: 60,
///             children: vec![],
///         },
///         BreakdownGroup {
///             label: "2024-02-29 (Thu)".to_string(),
///             minutes: 180,
///             children: vec![],
///         },
///     ],
/// }
/// ```
#[derive(Debug, Clone)]
pub struct BreakdownGroup {
    /// Human-readable label for this time period.
    ///
    /// Format depends on breakdown unit:
    /// - Day: `YYYY-MM-DD (DDD)` e.g., `2024-02-29 (Thu)`
    /// - Week: `YYYY-W##` e.g., `2024-W09`
    /// - Month: `YYYY-MM` e.g., `2024-02`
    /// - Year: `YYYY` e.g., `2024`
    pub label: String,

    /// Total minutes tracked in this time period.
    ///
    /// If children exist, this equals the sum of children's minutes.
    pub minutes: u32,

    /// Child groups for hierarchical breakdowns.
    ///
    /// Empty for leaf nodes (e.g., days in a week breakdown).
    /// Contains next level down in hierarchy (e.g., days within a week).
    pub children: Vec<BreakdownGroup>,
}

/// Hierarchical time breakdown report.
///
/// Aggregates time entries into a tree structure organized by calendar units
/// (day, week, month, or year). The structure depth depends on the breakdown unit:
///
/// - Day: 1 level (flat list)
/// - Week: 2 levels (weeks → days)
/// - Month: 2 levels (months → weeks)
/// - Year: 2 levels (years → months)
///
/// # Edge Cases Handled
///
/// - **ISO week boundaries**: Weeks spanning year boundaries correctly attributed
/// - **Leap years**: February 29 included in all breakdown levels
/// - **Sparse data**: Empty periods automatically omitted
/// - **Month-week overlaps**: Weeks spanning months appear in both months
///
/// # Examples
///
/// ```ignore
/// use time_tracker::domain::reporting::{BreakdownReport, BreakdownUnit};
///
/// // Create report from tracked time
/// let report = BreakdownReport::from_tracked_time(&tracked_time, BreakdownUnit::Week);
///
/// // Access total time
/// println!("Total: {}h {}m", report.total_minutes / 60, report.total_minutes % 60);
///
/// // Iterate through groups
/// for week in &report.groups {
///     println!("{}: {}m", week.label, week.minutes);
///     for day in &week.children {
///         println!("  {}: {}m", day.label, day.minutes);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct BreakdownReport {
    /// Top-level groups in the breakdown hierarchy.
    ///
    /// Sorted chronologically. Empty groups (minutes = 0) are omitted.
    /// Content depends on breakdown unit (days, weeks, months, or years).
    pub groups: Vec<BreakdownGroup>,

    /// Total minutes across all time entries in this report.
    ///
    /// Equals sum of all top-level groups' minutes.
    pub total_minutes: u32,

    /// Time period covered by this report.
    pub period: TrackingPeriod,
}

impl BreakdownReport {
    /// Creates a breakdown report from a flat list of time entries.
    ///
    /// **Note:** This constructor uses placeholder implementations. For production use,
    /// prefer [`from_tracked_time`](Self::from_tracked_time) which handles date grouping correctly.
    ///
    /// # Arguments
    ///
    /// * `entries` - Time entries to aggregate
    /// * `unit` - Calendar unit for aggregation (Day, Week, Month, Year)
    /// * `period` - Time period covered by this report
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let report = BreakdownReport::from_entries(
    ///     &time_entries,
    ///     BreakdownUnit::Week,
    ///     period,
    /// );
    /// ```
    #[must_use]
    pub fn from_entries(
        entries: &[TimeEntry],
        unit: BreakdownUnit,
        period: TrackingPeriod,
    ) -> Self {
        let total_minutes: u32 = entries.iter().map(|e| e.minutes).sum();
        let groups = match unit {
            BreakdownUnit::Day => break_down_by_day(entries),
            BreakdownUnit::Week => break_down_by_week(entries),
            BreakdownUnit::Month => break_down_by_month(entries),
            BreakdownUnit::Year => break_down_by_year(entries),
        };

        Self {
            groups,
            total_minutes,
            period,
        }
    }

    /// Creates a breakdown report from tracked time with pre-grouped entries by date.
    ///
    /// This is the primary constructor for breakdown reports. It properly handles:
    /// - ISO week year boundaries (e.g., 2021-01-01 in 2020-W53)
    /// - Leap years (February 29)
    /// - Empty period filtering
    /// - Week-month boundary overlaps
    ///
    /// # Arguments
    ///
    /// * `time_report` - Tracked time with entries grouped by date
    /// * `unit` - Calendar unit for aggregation (Day, Week, Month, Year)
    ///
    /// # Returns
    ///
    /// A hierarchical breakdown report with:
    /// - Groups sorted chronologically
    /// - Empty periods omitted
    /// - Correct ISO week attribution
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use time_tracker::domain::reporting::{BreakdownReport, BreakdownUnit};
    ///
    /// let report = BreakdownReport::from_tracked_time(&tracked_time, BreakdownUnit::Week);
    ///
    /// for week in &report.groups {
    ///     println!("{}: {}h {}m", week.label, week.minutes / 60, week.minutes % 60);
    /// }
    /// ```
    #[must_use]
    pub fn from_tracked_time(time_report: &TrackedTime, unit: BreakdownUnit) -> Self {
        let groups = match unit {
            BreakdownUnit::Day => break_down_by_day_with_dates(&time_report.entries_by_date),
            BreakdownUnit::Week => break_down_by_week_with_entries(&time_report.entries_by_date),
            BreakdownUnit::Month => break_down_by_month_with_entries(&time_report.entries_by_date),
            BreakdownUnit::Year => break_down_by_year_with_entries(&time_report.entries_by_date),
        };

        Self {
            groups,
            total_minutes: time_report.total_minutes,
            period: time_report.period,
        }
    }
}

fn break_down_by_day(_entries: &[TimeEntry]) -> Vec<BreakdownGroup> {
    // Placeholder - will use entries_by_date from TrackedTime in caller
    let by_day: std::collections::BTreeMap<NaiveDate, u32> = std::collections::BTreeMap::new();
    by_day
        .into_iter()
        .map(|(date, minutes)| BreakdownGroup {
            label: label_day(date),
            minutes,
            children: vec![],
        })
        .collect()
}

fn break_down_by_day_with_dates(
    entries_by_date: &std::collections::HashMap<NaiveDate, Vec<TimeEntry>>,
) -> Vec<BreakdownGroup> {
    let mut sorted_dates: Vec<_> = entries_by_date.keys().collect();
    sorted_dates.sort();

    sorted_dates
        .into_iter()
        .filter_map(|date| {
            let total_minutes: u32 = entries_by_date
                .get(date)
                .map_or(0, |entries| entries.iter().map(|e| e.minutes).sum::<u32>());
            if total_minutes > 0 {
                Some(BreakdownGroup {
                    label: label_day(*date),
                    minutes: total_minutes,
                    children: vec![],
                })
            } else {
                None
            }
        })
        .collect()
}

fn break_down_by_week_with_entries(
    entries_by_date: &std::collections::HashMap<NaiveDate, Vec<TimeEntry>>,
) -> Vec<BreakdownGroup> {
    let mut weeks_map: std::collections::BTreeMap<
        (i32, u32),
        std::collections::BTreeMap<NaiveDate, u32>,
    > = std::collections::BTreeMap::new();

    for (&date, entries) in entries_by_date {
        let week = date.iso_week();
        let week_key = (week.year(), week.week());
        let minutes: u32 = entries.iter().map(|e| e.minutes).sum();
        weeks_map.entry(week_key).or_default().insert(date, minutes);
    }

    weeks_map
        .into_iter()
        .map(|((year, week), days_in_week)| {
            let children: Vec<BreakdownGroup> = days_in_week
                .into_iter()
                .map(|(date, minutes)| BreakdownGroup {
                    label: label_day(date),
                    minutes,
                    children: vec![],
                })
                .collect();
            let minutes: u32 = children.iter().map(|c| c.minutes).sum();
            BreakdownGroup {
                label: label_week(year, week),
                minutes,
                children,
            }
        })
        .collect()
}

fn break_down_by_month_with_entries(
    entries_by_date: &std::collections::HashMap<NaiveDate, Vec<TimeEntry>>,
) -> Vec<BreakdownGroup> {
    let mut months_map: std::collections::BTreeMap<
        (i32, u32),
        std::collections::BTreeMap<(i32, u32), u32>,
    > = std::collections::BTreeMap::new();

    for (&date, entries) in entries_by_date {
        let year = date.year();
        let month = date.month();
        let week = date.iso_week();
        let week_key = (week.year(), week.week());
        let minutes: u32 = entries.iter().map(|e| e.minutes).sum();

        months_map
            .entry((year, month))
            .or_default()
            .entry(week_key)
            .and_modify(|m| *m += minutes)
            .or_insert(minutes);
    }

    months_map
        .into_iter()
        .map(|((year, month), weeks_in_month)| {
            let children: Vec<BreakdownGroup> = weeks_in_month
                .into_iter()
                .map(|((week_year, week), minutes)| BreakdownGroup {
                    label: label_week(week_year, week),
                    minutes,
                    children: vec![],
                })
                .collect();
            let minutes: u32 = children.iter().map(|c| c.minutes).sum();
            BreakdownGroup {
                label: label_month(year, month),
                minutes,
                children,
            }
        })
        .collect()
}

fn break_down_by_year_with_entries(
    entries_by_date: &std::collections::HashMap<NaiveDate, Vec<TimeEntry>>,
) -> Vec<BreakdownGroup> {
    let mut years_map: std::collections::BTreeMap<i32, std::collections::BTreeMap<u32, u32>> =
        std::collections::BTreeMap::new();

    for (&date, entries) in entries_by_date {
        let year = date.year();
        let month = date.month();
        let minutes: u32 = entries.iter().map(|e| e.minutes).sum();
        years_map
            .entry(year)
            .or_default()
            .entry(month)
            .and_modify(|m| *m += minutes)
            .or_insert(minutes);
    }

    years_map
        .into_iter()
        .map(|(year, months_in_year)| {
            let children: Vec<BreakdownGroup> = months_in_year
                .into_iter()
                .map(|(month, minutes)| BreakdownGroup {
                    label: label_month(year, month),
                    minutes,
                    children: vec![],
                })
                .collect();
            let minutes: u32 = children.iter().map(|c| c.minutes).sum();
            BreakdownGroup {
                label: label_year(year),
                minutes,
                children,
            }
        })
        .collect()
}

fn break_down_by_week(_entries: &[TimeEntry]) -> Vec<BreakdownGroup> {
    vec![]
}

fn break_down_by_month(_entries: &[TimeEntry]) -> Vec<BreakdownGroup> {
    vec![]
}

fn break_down_by_year(_entries: &[TimeEntry]) -> Vec<BreakdownGroup> {
    vec![]
}

fn label_day(date: NaiveDate) -> String {
    date.format("%Y-%m-%d (%a)").to_string()
}

fn label_week(year: i32, week: u32) -> String {
    format!("{year}-W{week:02}")
}

fn label_month(year: i32, month: u32) -> String {
    format!("{year}-{month:02}")
}

fn label_year(year: i32) -> String {
    format!("{year}")
}
