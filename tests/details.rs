pub mod test_helpers;
use test_helpers::*;

#[test]
fn details_flag_only_should_fail() {
    let some_content = r#"## TT 2020-01-01
- #context-1 1h Task A"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_details()
        .when_run()
        .should_fail()
        .expect_error("--details flag requires --tags to be specified");
}

#[test]
fn details_flag_should_require_tags_value() {
    let some_content = r#"## TT 2020-01-01
- #context-1 1h Task A"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_details()
        .with_tags_filter(&["context-1"])
        .when_run()
        .should_succeed();
}

#[test]
fn details_flag_should_only_include_tasks_of_given_context() {
    let some_content = r#"## TT 2020-01-01
- #context-1 1h Task A
- #context-2 2h Task A"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_details()
        .with_tags_filter(&["context-1"])
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m");
}

#[test]
fn details_flag_should_include_tasks() {
    let some_content = r#"## TT 2020-01-01
- #context-1 1h Task A"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_details()
        .with_tags_filter(&["context-1"])
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m");
}

#[test]
fn details_flag_should_only_include_tasks_of_given_context_and_duration() {
    let some_content = r#"## TT 2020-01-01
- #context-1 1h Task A"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_details()
        .with_tags_filter(&["context-1"])
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m");
}

#[test]
fn details_flag_with_multiple_tags_should_show_tasks_of_multiple_tags() {
    let some_content = r#"## TT 2020-01-01
- #context-1 1h Task A
- #context-2 2h Task B"#;

    CommandSpec::describe()
        .with_file_with_content(some_content)
        .with_details()
        .with_tags_filter(&["context-1", "context-2"])
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m")
        .expect_task_with_duration("Task B", "2h 00m");
}
