use crate::common::*;

#[test]
fn app_should_not_fail_when_no_outcomes_are_present() {
    let content = r#"## TT 2020-01-01
- #prj-1 1h Task A"#;

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_no_text("Outcomes:");
}

#[test]
#[ignore]
fn app_should_show_outcomes_in_summary() {
    let content = r#"## TT 2020-01-01
- #prj-1 ##outcome-xyz 1h Task A"#;

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_output("Outcomes:");
}
