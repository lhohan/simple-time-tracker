use crate::common::Cmd;

#[test]
fn markdown_overview_should_format_with_correct_structure() {
    Cmd::given()
        .output_format("markdown")
        .a_file_with_content(
            r"## TT 2020-01-01
            - #project 2h
            ",
        )
        .when_run()
        .should_succeed()
        .expect_output("# Time Tracking Report")
        .expect_output("## Overview")
        .expect_output("- **Period**:")
        .expect_output("- **Days Tracked**:")
        .expect_output("- **Hours per Day**:")
        .expect_output("- **Total Time**:")
        .expect_output("### Projects")
        .expect_output("- **project**:");
}

#[test]
fn markdown_details_should_format_with_correct_structure() {
    Cmd::given()
        .output_format("markdown")
        .project_filter("dev")
        .a_file_with_content(
            r"## TT 2020-01-01
            - #dev 1h task
            ",
        )
        .when_run()
        .should_succeed()
        .expect_output("# Time Tracking Details Report")
        .expect_output("## Project: dev")
        .expect_output("### Tasks")
        .expect_output("- **task**:");
}

#[test]
fn markdown_should_bold_format_project_names() {
    Cmd::given()
        .output_format("markdown")
        .a_file_with_content(
            r"## TT 2020-01-01
            - #work-project 3h
            ",
        )
        .when_run()
        .should_succeed()
        .expect_output("- **work-project**:  3h 00m (100%)");
}

#[test]
fn markdown_should_handle_special_characters_in_project_names() {
    Cmd::given()
        .output_format("markdown")
        .a_file_with_content(
            r"## TT 2020-01-01
            - #my-project_v2.0 1h
            - #client&company 1h
            ",
        )
        .when_run()
        .should_succeed()
        .expect_output("**my-project_v2.0**")
        .expect_output("**client&company**");
}
