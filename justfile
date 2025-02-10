# Build the 'tt' app with release flag
build:
    cargo build --release --bin tt

# Install 'tt' app locally
install:
    cargo install --path .

# Run the 'tt' app
run:
    cargo run -- --help

# Test the 'tt' app
test:
    cargo test

# Run tests on change continuously
test-w:
    cargo watch -c -x test

# Run tests with coverage
test-coverage:
    cargo tarpaulin -- --test-threads=1

# Run tests with coverage and open the report
test-coverage-report:
    cargo tarpaulin --out Html && open ./tarpaulin-report.html

# Run extensive Clippy linter checks
run-clippy:
    cargo clippy --all-targets -- -D clippy::all -D clippy::pedantic

# Clean the build artifacts
clean:
    cargo clean
