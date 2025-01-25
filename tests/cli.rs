use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
mod test_helpers;
use test_helpers::*;

#[test]
fn shows_help_information() -> Result<(), Box<dyn std::error::Error>> {
    CommandSpec::new().with_help().when_run().should_succeed();

    Ok(())
}

#[test]
fn test_basic_time_tracking() -> Result<(), Box<dyn std::error::Error>> {
    CommandSpec::new()
        .with_content(
            r#"
        ## TT 2025-01-15
        - #sport 30m
        - #coding 2p
        - #journaling 20m
        - #sport 1h
        "#,
        )
        .when_run()
        .should_succeed()
        .expect_project("sport")
        .taking("1h 30")
        .with_percentage("53")
        .expect_project("coding")
        .taking("1h  0m")
        .with_percentage("35")
        .expect_project("journaling")
        .taking("0h 20m")
        .with_percentage("12")
        .validate()?;

    Ok(())
}

#[test]
fn test_basic_time_tracking_basic_dsl() -> Result<(), Box<dyn std::error::Error>> {
    CommandSpec::new()
        .with_content(
            r#"
        ## TT 2025-01-15
        - #sport 30m
        - #coding 2p
        - #journaling 20m
        - #sport 1h
    "#,
        )
        .when_run()
        .should_succeed()
        .should_contain_project("sport", [("Duration", "1h 30m"), ("Percentage", "53")])
        .should_contain_project("coding", [("Duration", "1h  0m"), ("Percentage", "35")])
        .should_contain_project("journaling", [("Duration", "0h 20m"), ("Percentage", "12")]);

    Ok(())
}

#[test]
fn test_basic_time_tracking_old() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut c = Command::cargo_bin("tt")?;

    c.arg("--input")
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
        ));

    Ok(())
}

#[test]
fn test_verbose_output() -> Result<(), Box<dyn std::error::Error>> {
    CommandSpec::new()
        .with_verbose()
        .with_content(
            r#"## TT 2025-01-15
    - #test 30m"#,
        )
        .when_run()
        .should_succeed()
        .expect_processing_output();

    Ok(())
}

#[test]
fn should_only_process_entries_in_time_tracking_sections() -> Result<(), Box<dyn std::error::Error>>
{
    CommandSpec::new()
        .with_content(
            r#"# Random Header
Some random content
- #not-tracked 1h

## TT 2025-01-15
- #sport 1h
- #coding 2p

# Another Section
- #not-tracked 1h"#,
        )
        .when_run()
        .should_succeed()
        .expect_project("coding")
        .taking("1h  0m")
        .with_percentage("50")
        .expect_project("sport")
        .taking("1h  0m")
        .with_percentage("50")
        .validate()
}

#[test]
fn test_summary_statistics() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("day1.md");
    input_file.write_str(
        r#"## TT 2025-01-15
- #work 2h
- #exercise 2h
## TT 2025-01-16
- #work 3h
- #exercise 1h"#,
    )?;

    let mut c = Command::cargo_bin("tt")?;

    c.arg("--input")
        .arg(input_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "work.................. 5h  0m ( 63%)",
        ))
        .stdout(predicate::str::contains(
            "exercise.............. 3h  0m ( 38%)",
        ))
        .stdout(predicate::str::contains("2 days, 4.0 h/day"));

    Ok(())
}

#[test]
fn test_project_filter() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary test directory
    let temp = assert_fs::TempDir::new()?;

    // Create input file with test content
    let input_file = temp.child("test.md");
    input_file.write_str(
        r#"## TT 2025-01-15
- #dev #rust 2h implementing filters
- #dev 1h planning
- #sport 30m"#,
    )?;

    let mut c = Command::cargo_bin("tt")?;

    c.arg("--input")
        .arg(input_file.path())
        .arg("--project")
        .arg("dev")
        .assert()
        .success()
        .stdout(predicate::str::contains("Project: dev"))
        .stdout(predicate::str::contains("Total time:  3h"))
        .stdout(predicate::str::contains(
            "implementing filters.. 2h  0m (67%)",
        ))
        .stdout(predicate::str::contains(
            "planning.............. 1h  0m (33%)",
        ));

    Ok(())
}
