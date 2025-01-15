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

- #sport 30m
- #coding 2p
- #journaling 20m
- #sport 1h"#,
    )?;

    let mut cmd = Command::cargo_bin("time-tracker")?;

    cmd.arg("--input")
        .arg(input_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "journaling............ 0h 20m ( 12%)",
        ))
        .stdout(predicate::str::contains(
            "coding................ 1h  0m ( 35%)",
        ))
        .stdout(predicate::str::contains(
            "sport................. 1h 30m ( 53%)",
        ))
        .stdout(predicate::str::contains("Total................. 2h 50m"));

    Ok(())
}

#[test]
fn test_verbose_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("day1.md");
    input_file.write_str("- #test 30m")?;

    let mut cmd = Command::cargo_bin("time-tracker")?;

    cmd.arg("--input")
        .arg(input_file.path())
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processing file:"))
        .stdout(predicate::str::contains(
            "test.................. 0h 30m (100%)",
        ));

    Ok(())
}
