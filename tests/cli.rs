use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_basic_time_tracking() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary test directory
    let temp = assert_fs::TempDir::new()?;

    // Create input file with test content
    let input_file = temp.child("day1.md");
    input_file.write_str(
        r#"## TT 2025-01-15

- #journaling 20m
- #sport 1h"#,
    )?;

    // Run our CLI tool
    let mut cmd = Command::cargo_bin("time-tracker")?;
    cmd.arg(input_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("journaling: 20 minutes"))
        .stdout(predicate::str::contains("sport: 60 minutes"));

    Ok(())
}

#[test]
fn test_different_entries() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let input_file = temp.child("day2.md");
    input_file.write_str(
        r#"## TT 2025-01-15

- #coding 30m
- #reading 2h"#,
    )?;

    let mut cmd = Command::cargo_bin("time-tracker")?;
    cmd.arg(input_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("coding: 30 minutes"))
        .stdout(predicate::str::contains("reading: 120 minutes"));

    Ok(())
}
