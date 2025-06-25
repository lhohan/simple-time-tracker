use rstest::rstest;

use crate::common::Cmd;

#[test]
fn help_flag_should_show_help_information() {
    Cmd::given().help_flag().when_run().should_succeed();
}

#[rstest]
fn app_should_report_no_data_when_input_empty(#[values("", "## TT 2020-01-01")] empty_input: &str) {
    Cmd::given()
        .a_file_with_content(empty_input)
        .when_run()
        .should_succeed()
        .expect_no_data_found();
}

#[test]
fn app_should_generate_report_when_valid_entries_provided() {
    Cmd::given()
        .a_file_with_content(
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
fn app_should_ignore_non_entry_lines(
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

    Cmd::given()
        .a_file_with_content(&content)
        .when_run()
        .should_succeed()
        .expect_no_warnings();
}

#[test]
fn verbose_flag_should_show_processing_output() {
    Cmd::given()
        .verbose_flag()
        .a_file_with_content(
            r"## TT 2020-01-01
        - #test 30m",
        )
        .when_run()
        .should_succeed()
        .expect_processing_output();
}

#[test]
fn app_should_only_process_tt_sections() {
    Cmd::given()
        .a_file_with_content(
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
        .expect_project("sport")
        .taking("1h 00m")
        .validate();
}

#[test]
fn report_should_include_summary_statistics() {
    let content = r"## TT 2020-01-01
        - #work 2h
        - #exercise 2h
        ## TT 2020-01-02
        - #work 3h
        - #exercise 1h";

    Cmd::given()
        .a_file_with_content(content)
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
fn project_filter_should_show_project_details() {
    let content = r"## TT 2020-01-01
    - #dev #rust 2h implementing filters
    - #dev 1h planning
    - #sport 30m";

    Cmd::given()
        .a_file_with_content(content)
        .project_filter("dev")
        .when_run()
        .should_succeed()
        // expectations could be more precise
        .expect_output("Project: dev")
        .expect_task("implementing filters")
        .expect_task("planning")
        .expect_output("3h 00m total");
}

#[test]
fn project_filter_should_aggregate_duplicate_tasks() {
    let content = r"## TT 2020-01-01
    - #dev 1h My task
    - #dev 1h My task";

    Cmd::given()
        .a_file_with_content(content)
        .project_filter("dev")
        .when_run()
        .should_succeed()
        .expect_output("Project: dev")
        .expect_task_with_duration("My task", "2h 00m");
}

#[test]
fn project_filter_should_default_empty_task_descriptions() {
    let content = r"## TT 2020-01-01
    - #dev 2h";

    Cmd::given()
        .a_file_with_content(content)
        .project_filter("dev")
        .when_run()
        .should_succeed()
        // expectations could be more precise
        .expect_output("Project: dev")
        .expect_task("<no description>");
}

#[test]
fn report_should_show_start_date() {
    let content = r"## TT 2020-01-01
    - #dev 5h Task1
    ## TT 2020-01-02
    - #dev 5h Task2";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_start_date("2020-01-01");
}

#[test]
fn report_should_show_end_date() {
    let content = r"## TT 2020-01-01
    - #dev 5h Task1
    ## TT 2020-01-02
    - #dev 5h Task2";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_end_date("2020-01-02");
}

#[test]
fn from_filter_should_adjust_start_date() {
    let content = r"## TT 2020-01-01
    - #prj-1 3h Task 1
    ## TT 2020-02-01
    - #prj-2 2h Task 2";

    Cmd::given()
        .a_file_with_content(content)
        .from_date_filter("2020-01-02")
        .when_run()
        .should_succeed()
        .expect_start_date("2020-02-01");
}

#[test]
fn from_filter_should_show_report_description() {
    let content = r"## TT 2020-01-01
    - #prj-1 1h Task 1";

    Cmd::given()
        .a_file_with_content(content)
        .from_date_filter("2020-01-01")
        .when_run()
        .should_succeed()
        .expect_output("Time tracking report from 2020-01-01 until today");
}

#[test]
fn filters_should_work_when_combined() {
    let content = r"## TT 2020-01-01
    - #prj-1 3h Task 1
    ## TT 2020-01-02
    - #prj-1 7h Task 3
    - #prj-2 2h Task 2";

    Cmd::given()
        .a_file_with_content(content)
        .from_date_filter("2020-01-02")
        .project_filter("prj-1")
        .when_run()
        .should_succeed()
        .expect_start_date("2020-01-02")
        .expect_output("Project: prj-1")
        .expect_task_with_duration("Task 3", "7h 00m");
}

#[test]
fn report_should_show_date_range_when_no_period_filter() {
    let content = r"## TT 2020-01-15
    - #dev 1h Task1
    ## TT 2020-01-16
    - #dev 1h Task1";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_start_date("2020-01-15")
        .expect_end_date("2020-01-16")
        // todo?: expect_days(..), etc -> do when more extensive or targeted testing is aimed at this functionality
        .expect_output("2 days, 1.0 h/day,  2h 00m total");
}

#[rstest]
fn from_filter_should_fail_when_date_invalid(
    #[values("01-01-2020", "2020-00-01", "2020-01-00", "abc")] value: &str,
) {
    Cmd::given()
        .a_file_with_content(
            r"## TT 2020-01-01
    - #dev 1h Task1",
        )
        .at_date("2020-01-01")
        .from_date_filter(value)
        .when_run()
        .should_fail();
}

mod error_reporting {
    use crate::common::Cmd;

    #[test]
    fn app_should_report_warnings_when_entries_invalid() {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        - #dev Task 2 - Forgot to add time";

        Cmd::given()
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_project("dev")
            .taking("1h")
            .validate()
            .expect_warning("missing time: - #dev Task 2 - Forgot to add time");
    }

    #[test]
    fn app_should_show_line_numbers_when_errors_occur() {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        - #dev invalid time format
        - #dev 2h Task3";

        Cmd::given()
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_warning_at_line(3, "missing time: - #dev invalid time format");
    }

    #[test]
    fn app_should_show_line_number_when_date_invalid() {
        let content = r"## TT invalid-date
        - #dev 1h Task1";

        Cmd::given()
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_warning_at_line(1, "invalid date format: invalid-date");
    }

    #[test]
    fn app_should_only_warn_for_tt_sections() {
        let content = r"## A section title without teetee in
        - #dev 1h Task1";

        Cmd::given()
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_no_warnings();
    }

    #[test]
    fn app_should_not_warn_when_errors_outside_tt_sections() {
        Cmd::given()
            .a_file_with_content(
                r"# Random Header
                    - #1. If you don't get the requirements right",
            )
            .when_run()
            .should_succeed()
            .expect_no_warnings();
    }

    #[test]
    fn app_should_show_correct_line_numbers_for_multiple_errors() {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        - #dev Task2
        - #dev 2x Task3
        - #dev 1h Task4";

        Cmd::given()
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_warning_at_line(3, "missing time: - #dev Task2")
            .expect_warning_at_line(4, "missing time: - #dev 2x Task3");
    }

    #[test]
    fn app_should_show_file_name_in_warnings() {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        - #dev missing_time_entry";

        Cmd::given()
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_warning_with_file("test.md", "missing time: - #dev missing_time_entry");
    }
}

mod directory_processing {
    use crate::common::Cmd;

    #[test]
    fn app_should_process_multiple_files() {
        Cmd::given()
            .a_directory_containing_files(&[
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
    fn app_should_merge_same_project_across_files() {
        Cmd::given()
            .a_directory_containing_files(&[
                ("file1.md", "## TT 2020-01-15\n- #dev 1h Task1"),
                ("file2.md", "## TT 2020-01-15\n- #dev 2h Task2"),
            ])
            .when_run()
            .should_succeed()
            .expect_project("dev")
            .taking("3h 00m")
            .validate();
    }

    #[test]
    fn app_should_process_nested_directories() {
        Cmd::given()
            .a_directory_containing_files(&[
                ("2024/jan.md", "## TT 2024-01-01\n- #prj-1 2h Task1"),
                ("2025/jan.md", "## TT 2025-01-01\n- #prj-2 1h Task2"),
            ])
            .when_run()
            .should_succeed()
            .expect_project("prj-1")
            .expect_project("prj-2")
            .validate();
    }

    #[test]
    fn app_should_process_markdown_files() {
        Cmd::given()
            .a_directory_containing_files(&[("notes.md", "## TT 2024-01-01\n- #prj-1 1h Task1")])
            .when_run()
            .should_succeed()
            .expect_project("prj-1")
            .validate();
    }

    #[test]
    fn app_should_process_txt_files() {
        Cmd::given()
            .a_directory_containing_files(&[("notes.txt", "## TT 2024-01-01\n- #prj-1 1h Task1")])
            .when_run()
            .should_succeed()
            .expect_project("prj-1")
            .validate();
    }

    #[test]
    fn app_should_not_process_doc_files() {
        Cmd::given()
            .a_directory_containing_files(&[("ignored.doc", "## TT 2024-01-01\n- #prj-1 1h Task3")])
            .when_run()
            .should_succeed()
            .expect_no_data_found();
    }

    #[test]
    fn app_should_report_invalid_files_with_warnings() {
        Cmd::given()
            .a_directory_containing_files(&[
                ("valid.md", "## TT 2024-01-01\n- #prj-1 1h Task1"),
                ("invalid.md", "## TT invalid-date\n- #prj-2 2h Task2"),
            ])
            .when_run()
            .should_succeed()
            .expect_project("prj-1")
            .validate()
            .expect_warning_with_file("invalid.md", "invalid date format: invalid-date");
    }
}
