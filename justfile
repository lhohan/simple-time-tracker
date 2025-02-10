# Install 'tt' app locally
install:
  cargo install --path .

# Run the 'tt' app
run:
    cargo run --bin tt

# Build the 'tt' app
build:
    cargo build --bin tt

# Test the 'tt' app
test:
    cargo test

# Run tests on change continuously
test-w:
    cargo watch -c -x test

# Clean the build artifacts
clean:
    cargo clean

# Run extensive Clippy linter checks
run-clippy:
    cargo clippy --all-targets -- -D clippy::all -D clippy::pedantic
