#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]

use assert_cmd::Command;
use assert_fs::prelude::*;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Default, Clone)]
struct CommandArgs {
    args: Vec<String>,
}

impl CommandArgs {
    fn new() -> Self {
        Self::default()
    }

    fn add_flag(&mut self, flag: &str) {
        self.args.push(format!("--{flag}"));
    }

    fn add_option(&mut self, option: &str, value: &str) {
        self.args.push(format!("--{option}"));
        self.args.push(value.to_string());
    }

    fn into_vec(self) -> Vec<String> {
        self.args
    }
}

#[derive(Clone)]
pub struct CliStatisticsSpec {
    args: CommandArgs,
    input: Option<String>,
}

impl Default for CliStatisticsSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl CliStatisticsSpec {
    fn new() -> Self {
        Self {
            args: CommandArgs::new(),
            input: None,
        }
    }

    pub fn run_with_flag(mut self, flag: &str) -> Self {
        self.args.add_flag(flag);
        self
    }

    pub fn run_with_file(mut self, content: &str) -> Self {
        self.input = Some(content.to_string());
        self
    }

    pub fn run_with_filter(mut self, filter: &str, value: &str) -> Self {
        self.args.add_option(filter, value);
        self
    }

    pub fn when_executed(self) -> StatsCommandResult {
        let temp_stats_dir = Arc::new(
            assert_fs::TempDir::new().expect("Failed to create temporary stats directory"),
        );
        let stats_dir_path = temp_stats_dir.path().to_path_buf();

        let mut command = Command::cargo_bin("tt").expect("Failed to create cargo command");
        command.env("TT_STATS_DIR", &stats_dir_path);

        if let Some(content) = self.input {
            let input_file = temp_stats_dir.child("test.md");
            input_file
                .write_str(&content)
                .expect("Failed to write test input file");
            command.arg("--input").arg(input_file.path());
        }

        command.args(self.args.clone().into_vec());

        let output = command.assert();
        let stats_file = temp_stats_dir.child("stats.jsonl");

        StatsCommandResult {
            output,
            stats_file_path: stats_file.path().to_path_buf(),
            _temp_dir: temp_stats_dir,
        }
    }
}

pub struct CliStatistics;

impl CliStatistics {
    pub fn given() -> CliStatisticsSpec {
        CliStatisticsSpec::new()
    }
}

pub struct StatsCommandResult {
    pub output: assert_cmd::assert::Assert,
    pub stats_file_path: PathBuf,
    pub _temp_dir: Arc<assert_fs::TempDir>,
}

impl StatsCommandResult {
    pub fn should_record_stats(self) -> StatsAssertion {
        let exists = self.stats_file_path.exists();
        assert!(exists, "Stats file should exist at {:?}", self.stats_file_path);

        let content = std::fs::read_to_string(&self.stats_file_path)
            .expect("Failed to read stats file");

        let lines: Vec<&str> = content.lines().collect();
        assert!(!lines.is_empty(), "Stats file should contain at least one record");

        let last_record_str = lines.last().expect("Should have a record");
        let record: Value =
            serde_json::from_str(last_record_str).expect("Stats record should be valid JSON");

        StatsAssertion {
            record,
            _temp_dir: self._temp_dir,
        }
    }

    pub fn should_succeed(mut self) -> Self {
        self.output = self.output.success();
        self
    }

    pub fn should_fail(mut self) -> Self {
        self.output = self.output.failure();
        self
    }
}

pub struct StatsAssertion {
    record: Value,
    _temp_dir: Arc<assert_fs::TempDir>,
}

impl StatsAssertion {
    pub fn with_mode(self, expected_mode: &str) -> Self {
        let mode = self
            .record
            .get("mode")
            .and_then(|v| v.as_str())
            .expect("Stats record should have mode field");
        assert_eq!(mode, expected_mode, "Mode should be '{}'", expected_mode);
        self
    }

    pub fn with_success(self) -> Self {
        let success = self
            .record
            .get("success")
            .and_then(|v| v.as_bool())
            .expect("Stats record should have success field");
        assert!(success, "Record should indicate success");
        self
    }

    pub fn with_failure(self) -> Self {
        let success = self
            .record
            .get("success")
            .and_then(|v| v.as_bool())
            .expect("Stats record should have success field");
        assert!(!success, "Record should indicate failure");
        self
    }

    pub fn with_flags_used(self, expected_flags: Vec<&str>) -> Self {
        let flags_used = self
            .record
            .get("flags_used")
            .and_then(|v| v.as_array())
            .expect("Stats record should have flags_used array");

        let actual_flags: Vec<String> = flags_used
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let expected_flags_string: Vec<String> = expected_flags.iter().map(|s| s.to_string()).collect();

        let mut actual_sorted = actual_flags.clone();
        let mut expected_sorted = expected_flags_string.clone();
        actual_sorted.sort();
        expected_sorted.sort();

        assert_eq!(
            actual_sorted, expected_sorted,
            "Flags used should match. Expected: {:?}, Got: {:?}",
            expected_flags_string, actual_flags
        );
        self
    }

    pub fn having_flag(self, flag: &str) -> Self {
        let flags_used = self
            .record
            .get("flags_used")
            .and_then(|v| v.as_array())
            .expect("Stats record should have flags_used array");

        let has_flag = flags_used
            .iter()
            .any(|v| v.as_str().map_or(false, |s| s == flag));

        assert!(has_flag, "Flag '{}' should be in flags_used", flag);
        self
    }

    pub fn having_all_flags(self, flags: Vec<&str>) -> Self {
        let flags_used = self
            .record
            .get("flags_used")
            .and_then(|v| v.as_array())
            .expect("Stats record should have flags_used array");

        for expected_flag in flags {
            let has_flag = flags_used
                .iter()
                .any(|v| v.as_str().map_or(false, |s| s == expected_flag));
            assert!(
                has_flag,
                "Flag '{}' should be in flags_used",
                expected_flag
            );
        }
        self
    }

    pub fn not_having_flag(self, flag: &str) -> Self {
        let flags_used = self
            .record
            .get("flags_used")
            .and_then(|v| v.as_array())
            .expect("Stats record should have flags_used array");

        let has_flag = flags_used
            .iter()
            .any(|v| v.as_str().map_or(false, |s| s == flag));

        assert!(!has_flag, "Flag '{}' should not be in flags_used", flag);
        self
    }

    pub fn validate(self) {
    }
}
