mod test_helpers;
use rstest::rstest;
use test_helpers::*;

#[test]
fn shows_help_information() -> Result<(), Box<dyn std::error::Error>> {
    CommandSpec::new().with_help().when_run().should_succeed();

    Ok(())
}

#[rstest]
fn test_empties(
    #[values("", "## TT 2025-01-15")] empty_input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    CommandSpec::new()
        .with_content(empty_input)
        .when_run()
        .should_succeed()
        .expect_no_data_found();

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
        .expect_task("implementing filters")
        .expect_task("planning")
        .expect_output("Total time:  3h");

    Ok(())
}

#[test]
fn test_when_project_filter_should_total_task_with_same_name(
) -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-15
- #dev 1h My task
- #dev 1h My task"#;

    CommandSpec::new()
        .with_content(content)
        .with_project_filter("dev")
        .when_run()
        .should_succeed()
        .expect_output("Project: dev")
        .expect_task_with_duration("My task", "2h  0m");

    Ok(())
}

#[test]
fn test_when_project_filter_should_default_task_description_if_empty(
) -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-15
- #dev 2h"#;

    CommandSpec::new()
        .with_content(content)
        .with_project_filter("dev")
        .when_run()
        .should_succeed()
        // expectations could be more precise
        .expect_output("Project: dev")
        .expect_task("<no description>");

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

#[test]
fn test_report_should_include_interval_start() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #dev 5h Task1
## TT 2025-01-02
- #dev 5h Task2"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_start_date("2025-01-01");

    Ok(())
}

#[test]
fn test_report_should_include_interval_end() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #dev 5h Task1
## TT 2025-01-02
- #dev 5h Task2"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_end_date("2025-01-02");

    Ok(())
}

#[test]
fn test_date_filtering_from_date() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #prj-1 3h Task 1
## TT 2025-02-01
- #prj-2 2h Task 2"#;

    CommandSpec::new()
        .with_content(content)
        .with_from_date_filter("2025-01-02")
        .when_run()
        .should_succeed()
        .expect_start_date("2025-02-01");

    Ok(())
}

#[test]
fn test_combined_filtering_project_and_from_date() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #prj-1 3h Task 1
## TT 2025-01-02
- #prj-1 7h Task 3
- #prj-2 2h Task 2"#;

    CommandSpec::new()
        .with_content(content)
        .with_from_date_filter("2025-01-02")
        .with_project_filter("prj-1")
        .when_run()
        .should_succeed()
        .expect_start_date("2025-01-02")
        .expect_output("Project: prj-1")
        .expect_task_with_duration("Task 3", "7h  0m");

    Ok(())
}

#[test]
fn test_parsing_errors_should_show_line_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #dev 1h Task1
- #dev invalid time format
- #dev 2h Task3"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_warning_at_line(3, "missing time: - #dev invalid time format");

    Ok(())
}

#[test]
#[ignore] // todo: we should report wrong dates
fn test_invalid_date_format_shows_line_number() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT invalid-date
- #dev 1h Task1"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_warning_at_line(1, "invalid date format: invalid-date");

    Ok(())
}

#[test]
fn test_multiple_errors_show_correct_line_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"## TT 2025-01-01
- #dev 1h Task1
- #dev Task2
- #dev 2x Task3
- #dev 1h Task4"#;

    CommandSpec::new()
        .with_content(content)
        .when_run()
        .should_succeed()
        .expect_warning_at_line(3, "missing time: - #dev Task2")
        .expect_warning_at_line(4, "missing time: - #dev 2x Task3");

    Ok(())
}
