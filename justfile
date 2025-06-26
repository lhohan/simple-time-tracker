# Build the 'tt' app with release flag
build:
    cargo build --release --bin tt

# Install 'tt' app locally
install:
    cargo install --path .

# Check for warnings continuously
check-w:
    cargo watch -c -x check

# Run the 'tt' app
run:
    cargo run -- --help

# Test the 'tt' app
test:
    cargo nextest run

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

# Run extensive Clippy linter checks
run-clippy:
    cargo clippy --all-targets -- -D clippy::all -D clippy::pedantic

# Clean the build artifacts
clean:
    cargo clean
