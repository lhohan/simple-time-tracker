# Build the 'tt' app with release flag
build:
    cargo build --release --bin tt

# Build the 'tt-web' server with release flag
build-web:
    cargo build --release --bin tt-web

# Install 'tt' app locally
install:
    cargo install --path .

# Check for warnings continuously
check-w:
    cargo watch -c -x check

# Run the 'tt' app
run path period:
    cargo run -- -i "{{path}}" --period {{period}}

# Run the 'tt-web' server with optional data file path
web *args="":
    cargo run --bin tt-web -- {{args}}

# Run the 'tt-web' server with auto-reload on file changes
web-w *args="":
    cargo watch -c -x "run --bin tt-web -- {{args}}"

# Test the 'tt' app
test:
    cargo nextest run

# Test only the web dashboard
test-web:
    cargo nextest run --test web

# Run tests on change continuously
test-w:
    cargo watch -c -x "nextest run"

# Run tests with coverage
test-coverage:
    cargo llvm-cov nextest

# CI specific - Run tests with coverage
ci-test-coverage: test-coverage
    cargo llvm-cov report --lcov --output-path lcov.info

# Run tests with coverage and open the report
test-coverage-report:
    cargo llvm-cov nextest --open

# Run performance benchmarks (CLI parsing with large dataset)
bench:
    cargo bench

# Run benchmarks continuously on file changesqq
bench-w:
    cargo watch -c -x bench

# Format code with rustfmt
fmt:
    cargo fmt --all

# Check code formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run extensive Clippy linter checks
run-clippy:
    cargo clippy --all-targets -- -D clippy::all -D clippy::pedantic

# Clean the build artifacts
clean:
    cargo clean

# Test Serena MCP server startup
serena-mcp:
    ./scripts/serena-mcp

# Run all fuzz testing targets (10 seconds each, 40 seconds total)
fuzz:
    cargo +nightly fuzz run tag_fuzz -- -max_total_time=10
    cargo +nightly fuzz run time_fuzz -- -max_total_time=10
    cargo +nightly fuzz run description_fuzz -- -max_total_time=10
    cargo +nightly fuzz run multiline_fuzz -- -max_total_time=10

# Run all fuzz testing targets for longer sessions (2 minutes each)
fuzz-long:
    cargo +nightly fuzz run tag_fuzz -- -max_total_time=120
    cargo +nightly fuzz run time_fuzz -- -max_total_time=120
    cargo +nightly fuzz run description_fuzz -- -max_total_time=120
    cargo +nightly fuzz run multiline_fuzz -- -max_total_time=120

# Run all fuzz testing targets with custom time limit per target (in seconds)
fuzz-custom time:
    cargo +nightly fuzz run tag_fuzz -- -max_total_time={{time}}
    cargo +nightly fuzz run time_fuzz -- -max_total_time={{time}}
    cargo +nightly fuzz run description_fuzz -- -max_total_time={{time}}
    cargo +nightly fuzz run multiline_fuzz -- -max_total_time={{time}}
