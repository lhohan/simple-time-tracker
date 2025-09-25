#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]

use assert_cmd::Command;
use assert_fs::prelude::*;
use criterion::{measurement::WallTime, BatchSize, BenchmarkGroup, SamplingMode, Throughput};
use std::time::Duration;

/// DSL for creating performance benchmarks
#[derive(Debug, Clone)]
pub struct BenchmarkSpec {
    data_config: DataConfig,
    measurement_config: MeasurementConfig,
}

impl Default for BenchmarkSpec {
    fn default() -> Self {
        Self {
            data_config: DataConfig {
                days: 100,
                entries_per_day: 20,
            },
            measurement_config: MeasurementConfig::default(),
        }
    }
}

impl BenchmarkSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_data_size(mut self, days: usize, entries_per_day: usize) -> Self {
        self.data_config = DataConfig {
            days,
            entries_per_day,
        };
        self
    }

    pub fn add_to_group(&self, group: &mut BenchmarkGroup<WallTime>, label: &str) {
        let total_entries = (self.data_config.days * self.data_config.entries_per_day) as u64;

        group.sampling_mode(SamplingMode::Auto);
        group.sample_size(self.measurement_config.sample_size);
        group.measurement_time(self.measurement_config.measurement_time);
        group.warm_up_time(self.measurement_config.warm_up_time);
        group.throughput(Throughput::Elements(total_entries));

        group.bench_function(label, self.create_benchmark_fn());
    }

    fn create_benchmark_fn(&self) -> impl Fn(&mut criterion::Bencher) + '_ {
        move |bencher: &mut criterion::Bencher| {
            bencher.iter_batched(
                || self.generate_content(),
                |content| {
                    let output = Self::execute_cli(&content);
                    assert!(
                        output.status.success(),
                        "CLI command failed with status {:?}\nstderr: {}\nstdout: {}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr),
                        String::from_utf8_lossy(&output.stdout)
                    );
                },
                BatchSize::SmallInput,
            );
        }
    }
}

pub struct BenchmarkSuite;

impl BenchmarkSuite {
    pub fn large_dataset_benchmark() -> BenchmarkSpec {
        BenchmarkSpec::new().with_data_size(200, 40) // ~8k lines - sufficient to detect O(N²) issues
    }
}

#[derive(Debug, Clone)]
struct DataConfig {
    days: usize,
    entries_per_day: usize,
}

#[derive(Debug, Clone)]
struct MeasurementConfig {
    sample_size: usize,
    measurement_time: Duration,
    warm_up_time: Duration,
}

impl Default for MeasurementConfig {
    fn default() -> Self {
        Self {
            sample_size: 20,
            measurement_time: Duration::from_secs(8),
            warm_up_time: Duration::from_secs(3),
        }
    }
}

impl BenchmarkSpec {
    fn generate_content(&self) -> String {
        use std::fmt::Write;
        let mut content =
            String::with_capacity(self.data_config.days * self.data_config.entries_per_day * 40);

        for day_offset in 0..self.data_config.days {
            let day = 1 + (day_offset % 28);
            let _ = writeln!(&mut content, "## TT 2024-01-{day:02}");

            for _ in 0..self.data_config.entries_per_day {
                content.push_str("- #dev 1m some task text\n");
            }
        }

        content
    }

    fn execute_cli(content: &str) -> std::process::Output {
        let temp_dir = assert_fs::TempDir::new().expect("Failed to create temporary directory");
        let input_file = temp_dir.child("bench_input.md");
        input_file
            .write_str(content)
            .expect("Failed to write benchmark content to temp file");

        Command::cargo_bin("tt")
            .expect("Failed to create cargo command")
            .arg("--input")
            .arg(input_file.path())
            .output()
            .expect("CLI execution failed")
    }
}
