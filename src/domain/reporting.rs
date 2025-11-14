use std::collections::HashMap;
use std::hash::Hash;

use super::dates::{EndDate, StartDate};
use super::tags::Tag;
use super::{ParseError, PeriodRequested, TimeEntry};
use chrono::NaiveDate;
use chrono::{Datelike, IsoWeek};
use itertools::Itertools;
use serde::Serialize;

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
        let outcomes_summed = sum_outcomes(time_report, limit);

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

fn sum_outcomes(time_report: &TrackedTime, limit: Option<&OutputLimit>) -> Vec<TimeTotal> {
    let summed_outcomes = sum_time_by_key(time_report.entries.iter(), |entry| {
        entry
            .outcome
            .as_ref()
            .map(|outcome| outcome.description().to_string())
    });

    let summed_outcomes_sorted = summed_outcomes
        .into_iter()
        .map(|(outcome, minutes)| TimeTotal::new(outcome, minutes, time_report.total_minutes))
        .sorted_by(|a, b| {
            b.minutes
                .cmp(&a.minutes)
                .then(a.description.cmp(&b.description))
        });

    match limit {
        Some(OutputLimit::CumulativePercentageThreshold(threshold)) => {
            let total_minutes = f64::from(time_report.total_minutes);
            limit_number_of_entries(total_minutes, summed_outcomes_sorted, *threshold)
        }
        None => summed_outcomes_sorted.collect(),
    }
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakdownUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone)]
pub struct BreakdownGroup {
    pub label: String,
    pub minutes: u32,
    pub children: Vec<BreakdownGroup>,
}

#[derive(Debug)]
pub struct BreakdownReport {
    pub groups: Vec<BreakdownGroup>,
    pub total_minutes: u32,
    pub period: TrackingPeriod,
}

impl BreakdownReport {
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

#[derive(Debug, Clone, Serialize)]
pub struct TagUsage {
    pub tag: String,
    pub count: u32,
    pub percentage: u32,
    pub minutes: u32,
}

impl TagUsage {
    #[must_use]
    pub fn new(tag: String, count: u32, minutes: u32, total_minutes: u32) -> Self {
        Self {
            tag,
            count,
            percentage: calculate_percentage(minutes, total_minutes),
            minutes,
        }
    }
}

pub struct StatsReport {
    pub used_tags: Vec<TagUsage>,
    pub unused_tags: Vec<String>,
    pub period: TrackingPeriod,
    pub total_minutes: u32,
    pub total_entries: u32,
}

impl StatsReport {
    #[must_use]
    pub fn from_tracked_time(
        time_report: &TrackedTime,
        all_available_tags: Option<Vec<String>>,
    ) -> Self {
        let mut tag_usage_map: std::collections::HashMap<String, (u32, u32)> =
            std::collections::HashMap::new();

        for entry in &time_report.entries {
            for tag in &entry.tags {
                let tag_str = tag.to_string();
                let (count, minutes) = tag_usage_map.entry(tag_str).or_insert((0, 0));
                *count += 1;
                *minutes += entry.minutes;
            }
        }

        let mut used_tags: Vec<TagUsage> = tag_usage_map
            .into_iter()
            .map(|(tag, (count, minutes))| TagUsage::new(tag, count, minutes, time_report.total_minutes))
            .collect();

        used_tags.sort_by(|a, b| {
            b.minutes
                .cmp(&a.minutes)
                .then(b.count.cmp(&a.count))
                .then(a.tag.cmp(&b.tag))
        });

        let unused_tags = if let Some(all_tags) = all_available_tags {
            let used_tag_names: std::collections::HashSet<_> =
                used_tags.iter().map(|t| t.tag.clone()).collect();

            let mut unused: Vec<String> = all_tags
                .into_iter()
                .filter(|tag| !used_tag_names.contains(tag))
                .collect();
            unused.sort();
            unused
        } else {
            Vec::new()
        };

        StatsReport {
            used_tags,
            unused_tags,
            period: time_report.period,
            total_minutes: time_report.total_minutes,
            total_entries: time_report.entries.len() as u32,
        }
    }
}
