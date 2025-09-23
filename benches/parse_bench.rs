use criterion::{criterion_group, criterion_main, Criterion};

mod benchmark_dsl;
use benchmark_dsl::BenchmarkSuite;

/// Benchmark CLI parsing performance with large dataset
/// Tests the complete end-to-end CLI experience including I/O, parsing, and output generation
fn bench_parse_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_content");

    let spec = BenchmarkSuite::large_dataset_benchmark();
    spec.add_to_group(&mut group, "large_dataset");

    group.finish();
}

criterion_group!(benches, bench_parse_content);
criterion_main!(benches);
