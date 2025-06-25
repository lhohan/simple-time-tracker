use crate::common::*;

#[test]
fn limit_flag_should_show_projects_at_threshold() {
    let some_content = r#"## TT 2020-01-01
- #prj-1 9h Task A
- #prj-2 1h Task A
"#; // prj-1 is exactly 90% (9h out of 10h total)

    Cmd::given()
        .limit_flag()
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .validate()
        .expect_no_text("prj-2");
}

#[test]
fn limit_flag_should_filter_projects_above_threshold() {
    let some_content = r#"## TT 2020-01-01
- #prj-1 19h Task A
- #prj-2 1h Task A
"#; // prj-1 is 95% (19h out of 20h total)

    Cmd::given()
        .limit_flag()
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .validate()
        .expect_no_text("prj-2");
}

#[test]
fn limit_flag_should_show_first_project_above_threshold() {
    let some_content = r#"## TT 2020-01-01
- #prj-1 17h Task A
- #prj-2 2h Task A
- #prj-3 1h Task A
"#; // prj-1 is 80%, prj-2 goes to 95%

    Cmd::given()
        .limit_flag()
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .validate()
        .expect_project("prj-2")
        .validate()
        .expect_no_text("prj-3");
}

#[test]
fn app_should_show_all_projects_when_limit_not_specified() {
    let some_content = r#"## TT 2020-01-01
- #prj-1 9h Task A
- #prj-2 1h Task A
"#;

    Cmd::given()
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .validate()
        .expect_project("prj-2")
        .validate();
}
