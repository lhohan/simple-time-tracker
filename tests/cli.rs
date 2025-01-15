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

    cmd.arg(input_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("journaling: 20 minutes"))
        .stdout(predicate::str::contains("coding: 60 minutes"))
        .stdout(predicate::str::contains("sport: 90 minutes"));

    Ok(())
}
