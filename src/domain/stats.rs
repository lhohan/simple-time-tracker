use serde::Serialize;
use std::collections::{HashMap, HashSet};

use super::reporting::TrackedTime;

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
    pub total_minutes: u32,
    pub total_entries: u32,
}

impl StatsReport {
    #[must_use]
    pub fn from_tracked_time(
        time_report: &TrackedTime,
        all_available_tags: Option<Vec<String>>,
    ) -> Self {
        let mut tag_usage_map: HashMap<String, (u32, u32)> = HashMap::new();

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
            let used_tag_names: HashSet<_> = used_tags.iter().map(|t| t.tag.clone()).collect();

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
            total_minutes: time_report.total_minutes,
            total_entries: time_report.entries.len() as u32,
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
