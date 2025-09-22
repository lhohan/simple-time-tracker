use crate::common::*;

#[test]
fn details_flag_should_fail_when_no_tags_specified() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .details_flag()
        .a_file_with_content(some_content)
        .when_run()
        .should_fail()
        .expect_error("--details flag requires --tags to be specified");
}

#[test]
fn details_flag_should_succeed_when_tags_specified() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .details_flag()
        .tags_filter(&["tag-1"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed();
}

#[test]
fn details_flag_should_filter_tasks_by_tags() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A
- #tag-2 2h Task A";

    Cmd::given()
        .details_flag()
        .tags_filter(&["tag-1"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m");
}

#[test]
fn details_flag_should_show_task_details() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .details_flag()
        .tags_filter(&["tag-1"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m");
}

#[test]
fn details_flag_should_show_tasks_when_multiple_tags_specified() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A
- #tag-2 2h Task B";

    Cmd::given()
        .details_flag()
        .tags_filter(&["tag-1", "tag-2"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_task_with_duration("Task A", "1h 00m")
        .expect_task_with_duration("Task B", "2h 00m");
}
