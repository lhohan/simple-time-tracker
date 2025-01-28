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
        .validate();

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
        .validate();

    Ok(())
}

#[test]
fn test_summary_statistics() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-15
    - #work 2h
    - #exercise 2h
    ## TT 2025-01-16
    - #work 3h
    - #exercise 1h"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_project("work")
        .with_percentage("63")
        .taking("5h  0m")
        .expect_project("exercise")
        .with_percentage("38") // todo: sum of both percentages should be 100%
        .taking("3h  0m")
        .validate()
        .expect_output("2 days")
        .expect_output("4.0 h/day");

    Ok(())
}

#[test]
fn test_project_filter() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-15
- #dev #rust 2h implementing filters
- #dev 1h planning
- #sport 30m"#;

    CommandSpec::new()
        .with_content(content)
        .with_project_filter("dev")
        .when_run()
        .should_succeed()
        // expectations could be more precise
        .expect_output("Project: dev")
        .expect_output("implementing filters")
        .expect_output("planning")
        .expect_output("Total time:  3h");

    Ok(())
}

#[test]
fn test_when_errors_should_report_warnings() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #dev 1h Task1
- #dev Task 2 - Forgot to add time"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_project("dev")
        .taking("1h")
        .validate()
        .expect_warning("missing time: - #dev Task 2 - Forgot to add time");

    Ok(())
}
