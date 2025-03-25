use std::collections::HashMap;

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
}

impl TrackedTime {
    #[must_use]
    pub fn new(entries: Vec<TimeEntry>, start: StartDate, end: EndDate, days: u32) -> Self {
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            period: TrackingPeriod::new(start, end, days),
            total_minutes,
        }
    }

    pub fn tasks_tracked_for(&self, tags: Vec<Tag>) -> DetailReport {
        let mut per_tag_summaries = Vec::new();

        for tag in tags.iter() {
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
        let mut total_times_for_tasks = HashMap::new();

        let entries_for_tag = self.entries.iter().filter(|entry| entry.tags.contains(tag));
        for entry in entries_for_tag {
            let key = entry
                .description
                .clone()
                .unwrap_or_else(|| "<no description>".to_string());
            *total_times_for_tasks.entry(key).or_insert(0) += entry.minutes;
        }
        total_times_for_tasks.clone()
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
    summaries: Vec<ContextSummary>,
    period: TrackingPeriod,
    period_requested: Option<PeriodRequested>,
    total_minutes: u32,
}

impl OverviewReport {
    pub fn overview(
        time_report: &TrackedTime,
        limit: Option<OutputLimit>,
        period_requested: &Option<PeriodRequested>,
    ) -> Self {
        let summarized = summarize_entries(&time_report.entries);

        let summaries_sorted = summarized
            .into_iter()
            .map(|(project, minutes)| {
                ContextSummary::new(project, minutes, time_report.total_minutes)
            })
            .sorted_by(|a, b| {
                b.minutes
                    .cmp(&a.minutes)
                    .then(a.description.cmp(&b.description))
            });

        let entries = match limit {
            Some(OutputLimit::CummalitivePercentageThreshhold(threshold)) => {
                let total_minutes = time_report.total_minutes as f64;
                limit_number_of_entries(total_minutes, summaries_sorted, threshold)
            }
            None => summaries_sorted.collect(),
        };

        OverviewReport {
            summaries: entries,
            period: time_report.period,
            period_requested: period_requested.clone(),
            total_minutes: time_report.total_minutes,
        }
    }

    pub fn summaries(&self) -> &Vec<ContextSummary> {
        &self.summaries
    }

    pub fn period(&self) -> &TrackingPeriod {
        &self.period
    }

    pub fn period_requested(&self) -> &Option<PeriodRequested> {
        &self.period_requested
    }

    pub fn total_minutes(&self) -> u32 {
        self.total_minutes
    }
}

fn limit_number_of_entries(
    total_minutes: f64,
    summaries_sorted: std::vec::IntoIter<ContextSummary>,
    cumulative_percentage_threshold: f64,
) -> Vec<ContextSummary> {
    summaries_sorted
        .scan(0.0, |cumulative_percentage, entry| {
            let percentage = (entry.minutes as f64 / total_minutes) * 100.0;
            *cumulative_percentage += percentage;
            Some((entry, *cumulative_percentage))
        })
        .take_while(|(_, cumulative_percentage)| {
            *cumulative_percentage <= cumulative_percentage_threshold
        })
        .map(|(entry, _)| entry)
        .collect()
}

fn summarize_entries(entries: &[TimeEntry]) -> Vec<(String, u32)> {
    let mut summary = HashMap::new();

    for entry in entries {
        *summary.entry(entry.main_context().clone()).or_insert(0) += entry.minutes;
    }

    summary.into_iter().collect()
}

#[derive(Debug, Clone)]
pub struct ContextSummary {
    pub(crate) description: String,
    pub(crate) minutes: u32,
    pub(crate) percentage: u32,
}

impl ContextSummary {
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

    pub fn summaries(&self) -> &Vec<TaskSummariesForContext> {
        &self.summaries
    }

    pub fn period(&self) -> &TrackingPeriod {
        &self.period
    }

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

    pub fn context(&self) -> &Tag {
        &self.context
    }

    pub fn task_summaries(&self) -> &Vec<TaskSummary> {
        &self.entries
    }

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
    pub fn new(description: String, minutes: u32, total_minutes: u32) -> Self {
        Self {
            description,
            minutes,
            percentage_of_total: calculate_percentage(minutes, total_minutes),
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn calculate_percentage(minutes: u32, total_minutes: u32) -> u32 {
    ((f64::from(minutes) / f64::from(total_minutes)) * 100.0).round() as u32
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
    format!("of {}", date_str)
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
    CummalitivePercentageThreshhold(f64),
}
