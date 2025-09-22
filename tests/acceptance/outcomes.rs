use crate::common::*;

#[test]
fn report_should_not_show_outcomes_when_no_outcomes_are_present() {
    let content = r"## TT 2020-01-01
- #prj-1 1h Task A";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_no_text("Outcomes:");
}

#[test]
fn report_should_show_outcomes_when_outcomes_are_present() {
    let content = r"## TT 2020-01-01
- #prj-1 ##outcome-xyz 1h Task A";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_output("Outcomes:");
}

#[test]
fn report_should_show_sum_outcomes() {
    let content = r"## TT 2020-01-01
- #prj-1 ##same-outcome 1h Task A
- #prj-2 ##same-outcome 2h Task B
- #prj-1 ##not-same-outcome 4h Task B
";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_output("Outcomes:")
        .expect_outcome_with_duration("same-outcome", "3h 00m");
}
