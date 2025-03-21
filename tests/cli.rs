pub mod test_helpers;
use rstest::rstest;
use test_helpers::*;

#[test]
fn shows_help_information() {
    CommandSpec::new().with_help().when_run().should_succeed();
}

#[rstest]
fn test_empties(#[values("", "## TT 2020-01-01")] empty_input: &str) {
    CommandSpec::new()
        .with_file(empty_input.trim())
        .when_run()
        .should_succeed()
        .expect_no_data_found();
}

#[test]
fn test_basic_time_tracking() {
    CommandSpec::new()
        .with_file(
            r"
        ## TT 2020-01-01
        - #prj-1 30m
        - #prj-2  2p
        - #prj-3  20m
        - #prj-1  1h
        ",
        )
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .taking("1h 30")
        .with_percentage("53")
        .expect_project("prj-2")
        .taking("1h 00m")
        .with_percentage("35")
        .expect_project("prj-3")
        .taking("0h 20m")
        .with_percentage("12")
        .validate()
        .expect_no_warnings();
}

#[rstest]
fn test_non_entries_should_be_ignored(
    #[values(
    "", // empty line
    "    ", // empty line with spaces
    "some text in a time tracking section", // time tracking section should allow for some text that are not entries
)]
    non_entry_input: &str,
) {
    let content = format!(
        r"
        ## TT 2020-01-01
        {non_entry_input}
        - #prj-1 1h
        {non_entry_input}
        ",
    );

    CommandSpec::new()
        .with_file(&content)
        .when_run()
        .should_succeed()
        .expect_no_warnings();
}

#[test]
fn test_verbose_output() {
    CommandSpec::new()
        .with_verbose()
        .with_file(
            r"## TT 2020-01-01
    - #test 30m",
        )
        .when_run()
        .should_succeed()
        .expect_processing_output();
}

#[test]
fn should_only_process_entries_in_time_tracking_sections() {
    CommandSpec::new()
        .with_file(
            r"# Random Header
Some random content
- #coding 1h

## TT 2020-01-01
- #sport 1h
- #coding 2p

# Another Section
- #sport 1h",
        )
        .when_run()
        .should_succeed()
        .expect_project("coding")
        .taking("1h 00m")
        .with_percentage("50")
        .expect_project("sport")
        .taking("1h 00m")
        .with_percentage("50")
        .validate();
}

#[test]
fn when_entry_has_error_and_not_in_time_tracking_section_should_not_report_warning() {
    CommandSpec::new()
        .with_file(
            r"# Random Header
            - #1. If you don’t get the requirements right",
        )
        .when_run()
        .should_succeed()
        .expect_no_warnings();
}

#[test]
fn test_summary_statistics() {
    let content = r"## TT 2020-01-01
    - #work 2h
    - #exercise 2h
    ## TT 2020-01-16
    - #work 3h
    - #exercise 1h";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_project("work")
        .with_percentage("63")
        .taking("5h 00m")
        .expect_project("exercise")
        .with_percentage("38") // todo: sum of both percentages should be 100%
        .taking("3h 00m")
        .validate()
        .expect_output("2 days")
        .expect_output("4.0 h/day");
}

#[test]
fn test_project_filter() {
    let content = r"## TT 2020-01-01
- #dev #rust 2h implementing filters
- #dev 1h planning
- #sport 30m";

    CommandSpec::new()
        .with_file(content)
        .with_project_filter("dev")
        .when_run()
        .should_succeed()
        // expectations could be more precise
        .expect_output("Project: dev")
        .expect_task("implementing filters")
        .expect_task("planning")
        .expect_output("3h 00m total");
}

#[test]
fn test_when_project_filter_should_total_task_with_same_name() {
    let content = r"## TT 2020-01-01
- #dev 1h My task
- #dev 1h My task";

    CommandSpec::new()
        .with_file(content)
        .with_project_filter("dev")
        .when_run()
        .should_succeed()
        .expect_output("Project: dev")
        .expect_task_with_duration("My task", "2h 00m");
}

#[test]
fn test_when_project_filter_should_default_task_description_if_empty() {
    let content = r"## TT 2020-01-01
- #dev 2h";

    CommandSpec::new()
        .with_file(content)
        .with_project_filter("dev")
        .when_run()
        .should_succeed()
        // expectations could be more precise
        .expect_output("Project: dev")
        .expect_task("<no description>");
}

#[test]
fn test_when_errors_should_report_warnings() {
    let content = r"## TT 2020-01-01
- #dev 1h Task1
- #dev Task 2 - Forgot to add time";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_project("dev")
        .taking("1h")
        .validate()
        .expect_warning("missing time: - #dev Task 2 - Forgot to add time");
}

#[test]
fn test_report_should_include_interval_start() {
    let content = r"## TT 2020-01-01
- #dev 5h Task1
## TT 2020-01-02
- #dev 5h Task2";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_start_date("2020-01-01");
}

#[test]
fn test_report_should_include_interval_end() {
    let content = r"## TT 2020-01-01
- #dev 5h Task1
## TT 2020-01-02
- #dev 5h Task2";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_end_date("2020-01-02");
}

#[test]
fn test_date_filtering_from_date_shows_correct_start_date() {
    let content = r"## TT 2020-01-01
- #prj-1 3h Task 1
## TT 2020-02-01
- #prj-2 2h Task 2";

    CommandSpec::new()
        .with_file(content)
        .with_from_date_filter("2020-01-02")
        .when_run()
        .should_succeed()
        .expect_start_date("2020-02-01");
}

#[test]
fn test_date_filtering_from_date_shows_correct_description() {
    let content = r"## TT 2020-01-01
- #prj-1 1h Task 1";

    CommandSpec::new()
        .with_file(content)
        .with_from_date_filter("2020-01-01")
        .when_run()
        .should_succeed()
        .expect_output("Time tracking report from 2020-01-01 until today");
}

#[test]
fn test_combined_filtering_project_and_from_date() {
    let content = r"## TT 2020-01-01
- #prj-1 3h Task 1
## TT 2020-01-02
- #prj-1 7h Task 3
- #prj-2 2h Task 2";

    CommandSpec::new()
        .with_file(content)
        .with_from_date_filter("2020-01-02")
        .with_project_filter("prj-1")
        .when_run()
        .should_succeed()
        .expect_start_date("2020-01-02")
        .expect_output("Project: prj-1")
        .expect_task_with_duration("Task 3", "7h 00m");
}

#[test]
fn test_parsing_errors_should_show_line_numbers() {
    let content = r"## TT 2020-01-01
- #dev 1h Task1
- #dev invalid time format
- #dev 2h Task3";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_warning_at_line(3, "missing time: - #dev invalid time format");
}

#[test]
fn test_invalid_date_format_shows_line_number() {
    let content = r"## TT invalid-date
- #dev 1h Task1";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_warning_at_line(1, "invalid date format: invalid-date");
}

#[test]
fn test_only_warnings_for_sections_with_tt_in() {
    let content = r"## A section title without teetee in
- #dev 1h Task1";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_no_warnings();
}

#[test]
fn test_multiple_errors_show_correct_line_numbers() {
    let content = r"## TT 2020-01-01
- #dev 1h Task1
- #dev Task2
- #dev 2x Task3
- #dev 1h Task4";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_warning_at_line(3, "missing time: - #dev Task2")
        .expect_warning_at_line(4, "missing time: - #dev 2x Task3");
}

#[test]
fn test_errors_show_file_name() {
    let content = r"## TT 2020-01-01
- #dev 1h Task1
- #dev missing_time_entry";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_warning_with_file("test.md", "missing time: - #dev missing_time_entry");
}

#[test]
fn test_process_directory() {
    CommandSpec::new()
        .with_directory_containing_files(&[
            ("file1.md", "## TT 2024-01-01\n- #prj-1 2h Task1"),
            ("file2.md", "## TT 2020-01-01\n- #prj-2 1h Task2"),
        ])
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .taking("2h 00m")
        .expect_project("prj-2")
        .taking("1h 00m")
        .validate();
}

#[test]
fn test_process_directory_with_multiple_files_should_merge_days() {
    CommandSpec::new()
        .with_directory_containing_files(&[
            ("file1.md", "## TT 2020-01-15\n- #dev 1h Task1"),
            ("file2.md", "## TT 2020-01-15\n- #dev 2h Task2"), // same day, same project!
        ])
        .when_run()
        .should_succeed()
        .expect_project("dev")
        .taking("3h 00m")
        .validate();
}

#[test]
fn test_process_nested_directories() {
    CommandSpec::new()
        .with_directory_containing_files(&[
            ("2024/jan.md", "## TT 2024-01-01\n- #prj-1 2h Task1"),
            ("2025/jan.md", "## TT 2020-01-01\n- #prj-1 1h Task2"),
        ])
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .taking("3h 00m") // Should combine times across directories
        .validate();
}

#[test]
fn test_process_directory_file_filtering() {
    CommandSpec::new()
        .with_directory_containing_files(&[
            ("notes.md", "## TT 2024-01-01\n- #prj-1 2h Task1"),
            ("ignored.txt", "## TT 2024-01-01\n- #prj-2 1h Task2"),
            ("also_ignored.doc", "## TT 2024-01-01\n- #prj-3 1h Task3"),
        ])
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .taking("2h 00m")
        .validate();
}

#[test]
fn test_directory_processing_with_invalid_files() {
    CommandSpec::new()
        .with_directory_containing_files(&[
            ("valid.md", "## TT 2024-01-01\n- #prj-1 2h Task1"),
            ("invalid.md", "## TT invalid-date\n- #prj-2 1h Task2"),
        ])
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .taking("2h 00m")
        .validate()
        .expect_warning_with_file("invalid.md", "invalid date format: invalid-date");
}

#[test]
fn report_header_format_should_include_date_when_no_period_filter() {
    let content = r"## TT 2020-01-15
- #dev 1h Task1
## TT 2020-01-16
- #dev 1h Task1";

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_start_date("2020-01-15")
        .expect_end_date("2020-01-16")
        // todo?: expect_days(..), etc -> do when more extensive or targeted testing is aimed at this functionality
        .expect_output("2 days, 1.0 h/day,  2h 00m total");
}

#[rstest]
fn invalid_from(#[values("01-01-2020", "2020-00-01", "2020-01-00", "abc")] value: &str) {
    CommandSpec::new()
        .with_file(
            r"## TT 2020-01-01
- #dev 1h Task1",
        )
        .at_date("2020-01-01")
        .with_from_date_filter(value)
        .when_run()
        .should_fail();
}
