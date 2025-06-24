use crate::common::*;

#[test]
fn limit_flag_should_work() {
    let some_content = r#"## TT 2020-01-01
- #tag1 1h Task A"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_limit()
        .when_run()
        .should_succeed();
}

#[test]
fn limit_flag_should_show_only_until_percentage_threshold_of_90() {
    let some_content = r#"## TT 2020-01-01
- #prj-1 9h Task A
- #prj-2 1h Task A
"#; // 9 is 90% of 10.

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_limit()
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .validate()
        .expect_no_text("prj-2");
}

#[test]
fn without_limit_flag_should_show_all() {
    let some_content = r#"## TT 2020-01-01
- #prj-1 9h Task A
- #prj-2 1h Task A
"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_project("prj-1")
        .validate()
        .expect_project("prj-2")
        .validate();
}
