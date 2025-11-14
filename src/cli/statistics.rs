use crate::cli::Args;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatRecord {
    pub timestamp: String,
    pub mode: String,
    pub flags_used: Vec<String>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

pub struct StatisticsCollector;

impl StatisticsCollector {
    pub fn from_args(args: &Args) -> StatRecord {
        let mut flags_used = Vec::new();

        if args.input.is_some() {
            flags_used.push("input".to_string());
        }
        if args.verbose {
            flags_used.push("verbose".to_string());
        }
        if args.limit {
            flags_used.push("limit".to_string());
        }
        if args.details {
            flags_used.push("details".to_string());
        }
        if args.project.is_some() {
            flags_used.push("project".to_string());
        }
        if args.tags.is_some() {
            flags_used.push("tags".to_string());
        }
        if args.exclude_tags.is_some() {
            flags_used.push("exclude_tags".to_string());
        }
        if args.from.is_some() {
            flags_used.push("from".to_string());
        }
        if args.period.is_some() {
            flags_used.push("period".to_string());
        }
        if args.format.as_deref() != Some("text") {
            flags_used.push("format".to_string());
        }
        if args.breakdown.is_some() {
            flags_used.push("breakdown".to_string());
        }
        if args.web {
            flags_used.push("web".to_string());
        }
        if args.port != 3000 {
            flags_used.push("port".to_string());
        }
        if args.host != "127.0.0.1" {
            flags_used.push("host".to_string());
        }

        let mode = if args.web { "web" } else { "cli" };

        StatRecord {
            timestamp: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            mode: mode.to_string(),
            flags_used,
            success: true,
            error_type: None,
        }
    }

    pub fn with_failure(mut record: StatRecord, error_type: String) -> StatRecord {
        record.success = false;
        record.error_type = Some(error_type);
        record
    }
}

pub fn write_stat_record(record: &StatRecord) -> std::io::Result<()> {
    let stats_dir = get_stats_dir();
    fs::create_dir_all(&stats_dir)?;

    let stats_file = stats_dir.join("stats.jsonl");
    let json_line = serde_json::to_string(record)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut content = if stats_file.exists() {
        fs::read_to_string(&stats_file)?
    } else {
        String::new()
    };

    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    content.push_str(&json_line);
    content.push('\n');

    fs::write(&stats_file, content)?;
    Ok(())
}

fn get_stats_dir() -> PathBuf {
    if let Ok(stats_dir) = std::env::var("TT_STATS_DIR") {
        PathBuf::from(stats_dir)
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".time-tracker")
    }
}
