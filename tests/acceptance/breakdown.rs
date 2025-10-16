use crate::common::*;

#[test]
fn breakdown_requires_tags_or_project() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .breakdown_flag("day")
        .a_file_with_content(some_content)
        .when_run()
        .should_fail()
        .expect_error("--breakdown flag requires --tags or --project to be specified");
}

#[test]
fn breakdown_day_should_succeed_with_tags() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed();
}

#[test]
fn breakdown_day_should_show_day_entries() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A
- #tag-1 30m Task B";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01-01")
        .expect_output("1h 30m");
}

#[test]
fn breakdown_day_markdown_format() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .output_format("markdown")
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("# Time Breakdown Report")
        .expect_output("2020-01-01")
        .expect_output("1h 00m");
}

#[test]
fn breakdown_day_chronological_ordering() {
    let some_content = r"## TT 2020-01-03
- #tag-1 1h Task C

## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-02
- #tag-1 1h Task B";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-03")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        // Verify dates appear in strict chronological order using regex pattern with multiline
        .expect_output_pattern(r"2020-01-01(?s).*2020-01-02(?s).*2020-01-03");
}

#[test]
fn breakdown_day_human_friendly_labels() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("Wed");
}

#[test]
fn breakdown_day_omits_zero_entry_dates() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-02

## TT 2020-01-03
- #tag-1 1h Task C";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-03")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_no_text("2020-01-02");
}
