use rstest::rstest;

use crate::common::*;

#[test]
fn breakdown_should_require_tags_or_project() {
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
fn breakdown_day_should_succeed_with_project() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .breakdown_flag("day")
        .project_filter("tag-1")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed();
}

#[rstest]
fn breakdown_day_should_show_day_entries(#[values("day", "d")] flag: &str) {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A
- #tag-1 30m Task B";

    Cmd::given()
        .breakdown_flag(flag)
        .tags_filter(&["tag-1"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01-01 (")
        .expect_output("1h 30m");
}

#[rstest]
fn breakdown_week_should_show_week_entries(#[values("week", "w")] flag: &str) {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-02
- #tag-1 30m Task B

## TT 2020-01-03
- #tag-1 30m Task C";

    Cmd::given()
        .breakdown_flag(flag)
        .tags_filter(&["tag-1"])
        .at_date("2020-01-03")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-W01")
        .expect_output("2h 00m");
}

#[rstest]
fn breakdown_month_should_show_month_entries(#[values("month", "m")] flag: &str) {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-08
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag(flag)
        .tags_filter(&["tag-1"])
        .at_date("2020-01-08")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01..")
        .expect_output("3h 00m");
}

#[rstest]
fn breakdown_year_should_show_year_entries(#[values("year", "y")] flag: &str) {
    let some_content = r"## TT 2020-01-15
- #tag-1 1h Task A

## TT 2020-02-20
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag(flag)
        .tags_filter(&["tag-1"])
        .at_date("2020-02-20")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020")
        .expect_output("3h 00m");
}

#[test]
fn breakdown_with_markdown_format_should_show_markdown_output() {
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
fn breakdown_day_should_order_chronologically() {
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
        .expect_output_pattern(r"2020-01-01 \((?s).*2020-01-02 \((?s).*2020-01-03 \(");
}

#[test]
fn breakdown_by_day_should_show_human_friendly_labels() {
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
fn breakdown_by_day_should_omit_zero_entry_dates() {
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
        .expect_no_text("2020-01-02 (");
}

#[test]
fn breakdown_by_week_should_show_hierarchical_weeks_days() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-02
- #tag-1 30m Task B

## TT 2020-01-03
- #tag-1 30m Task C";

    Cmd::given()
        .breakdown_flag("week")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-03")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-W01") // show week number
        .expect_output("2020-01-01 (") // show day
        .expect_output("2020-01-02 (") // show day
        .expect_output("2020-01-03 (") // show day
        .expect_output("2h 00m"); // show total time
}

#[test]
fn breakdown_by_month_should_show_hierarchical_months_weeks() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-08
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("month")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-08")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01..") // shows month
        .expect_output("3h 00m") // shows month total time
        .expect_output("2020-W01") // shows week 1
        .expect_output("2020-W02"); // shows week 2
}

#[test]
fn breakdown_by_month_should_not_show_days() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-15
- #tag-1 1h Task B";

    Cmd::given()
        .breakdown_flag("month")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-15")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_no_text("2020-01-01 (")
        .expect_no_text("2020-01-15 (");
}

#[test]
fn breakdown_by_year_should_show_hierarchical_years_months() {
    let some_content = r"## TT 2020-01-15
- #tag-1 1h Task A

## TT 2020-02-20
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("year")
        .tags_filter(&["tag-1"])
        .at_date("2020-02-20")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020")
        .expect_output("2020-01")
        .expect_output("2020-02")
        .expect_output("3h 00m");
}

#[test]
fn breakdown_by_year_should_not_show_weeks() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-08
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("year")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-08")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020") // show year
        .expect_output("2020-01") // show month
        .expect_no_text("2020-W01"); // don't show weeks
}

#[test]
fn breakdown_auto_with_day_period_should_show_weeks() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A
- #tag-1 30m Task B";

    Cmd::given()
        .breakdown_flag("auto")
        .period_filter("2020-01-01")
        .tags_filter(&["tag-1"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-W01") // show week (one level above day period)
        .expect_output("2020-01-01 ("); // show days
}

#[test]
fn breakdown_auto_with_week_period_should_show_months() {
    let some_content = r"## TT 2020-01-06
- #tag-1 1h Task A

## TT 2020-01-07
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("auto")
        .period_filter("this-week")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-07")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01..") // show month (one level above week period)
        .expect_output("2020-W02") // show weeks
        .expect_no_text("2020-01-06 (") // don't show days
        .expect_output("3h 00m");
}

#[test]
fn breakdown_auto_with_month_period_should_show_years() {
    let some_content = r"## TT 2020-02-01
- #tag-1 1h Task A

## TT 2020-02-15
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("auto")
        .period_filter("this-month")
        .tags_filter(&["tag-1"])
        .at_date("2020-02-15")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020..") // show year (one level above month period)
        .expect_output("2020-02..") // show months
        .expect_no_text("2020-W01") // don't show weeks
        .expect_output("3h 00m");
}

#[test]
fn breakdown_auto_with_year_period_should_show_years_and_months() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-02-01
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("auto")
        .period_filter("2020")
        .tags_filter(&["tag-1"])
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020..") // show year (stays at year for year period)
        .expect_output("2020-01..") // show months
        .expect_output("2020-02..")
        .expect_no_text("2020-W01"); // don't show weeks
}

#[test]
fn breakdown_by_week_should_handle_iso_week_boundary_year_transition() {
    // ISO week 53 in 2020 includes Dec 28-31, 2020 (and Jan 1, 2021)
    let some_content = r"## TT 2020-12-28
- #tag-1 1h Task A

## TT 2020-12-31
- #tag-1 1h Task B

## TT 2021-01-01
- #tag-1 1h Task C";

    Cmd::given()
        .breakdown_flag("week")
        .tags_filter(&["tag-1"])
        .at_date("2021-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-W53") // ISO week 53 spans years
        .expect_output("2020-12-28 (")
        .expect_output("2020-12-31 (")
        .expect_output("2021-01-01 (")
        .expect_output("3h 00m");
}

#[test]
fn breakdown_by_month_should_handle_year_transition() {
    let some_content = r"## TT 2020-12-15
- #tag-1 1h Task A

## TT 2021-01-15
- #tag-1 2h Task B

## TT 2021-02-15
- #tag-1 4h Task C";

    Cmd::given()
        .breakdown_flag("month")
        .tags_filter(&["tag-1"])
        .at_date("2021-02-15")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-12..")
        .expect_output("2021-01..")
        .expect_output("2021-02..")
        .expect_output("7h 00m");
}

#[test]
fn breakdown_by_year_should_handle_multi_year_entries() {
    let some_content = r"## TT 2019-06-15
- #tag-1 1h Task A

## TT 2020-03-20
- #tag-1 2h Task B

## TT 2021-11-10
- #tag-1 3h Task C";

    Cmd::given()
        .breakdown_flag("year")
        .tags_filter(&["tag-1"])
        .at_date("2021-11-10")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2019..") // show all three years
        .expect_output("2020..")
        .expect_output("2021..")
        .expect_output("6h 00m"); // total time across years
}

#[test]
fn breakdown_by_week_should_handle_week_1_starting_in_previous_year() {
    // ISO week 1 of 2024 starts on 2024-01-01 (Monday)
    // but ISO week 1 of 2021 starts on 2021-01-04 (Monday)
    // meaning 2021-01-01 through 2021-01-03 belong to 2020-W53
    let some_content = r"## TT 2021-01-01
- #tag-1 1h Task A

## TT 2021-01-04
- #tag-1 2h Task B

## TT 2021-01-05
- #tag-1 1h Task C";

    Cmd::given()
        .breakdown_flag("week")
        .tags_filter(&["tag-1"])
        .at_date("2021-01-05")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-W53") // Jan 1-3, 2021 are in week 53 of 2020
        .expect_output("2021-W01") // Jan 4-5, 2021 are in week 1 of 2021
        .expect_output("2021-01-01 (")
        .expect_output("2021-01-04 (")
        .expect_output("2021-01-05 (")
        .expect_output("4h 00m");
}

#[test]
fn breakdown_by_day_should_handle_leap_year_february_29() {
    let some_content = r"## TT 2024-02-28
- #tag-1 1h Task A

## TT 2024-02-29
- #tag-1 2h Task B

## TT 2024-03-01
- #tag-1 1h Task C";

    Cmd::given()
        .breakdown_flag("day")
        .tags_filter(&["tag-1"])
        .at_date("2024-03-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2024-02-28 (")
        .expect_output("2024-02-29 (") // Leap day should appear
        .expect_output("2024-03-01 (")
        .expect_output("4h 00m");
}

#[test]
fn breakdown_by_week_should_handle_leap_year_february_29() {
    // 2024-02-29 falls in week 9 of 2024
    let some_content = r"## TT 2024-02-26
- #tag-1 1h Task A

## TT 2024-02-29
- #tag-1 3h Task B

## TT 2024-03-01
- #tag-1 2h Task C";

    Cmd::given()
        .breakdown_flag("week")
        .tags_filter(&["tag-1"])
        .at_date("2024-03-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2024-W09")
        .expect_output("2024-02-26 (")
        .expect_output("2024-02-29 (") // Leap day in week breakdown
        .expect_output("2024-03-01 (")
        .expect_output("6h 00m");
}

#[test]
fn breakdown_by_month_should_omit_empty_weeks() {
    // Month with entries in week 1 and week 3, but not week 2
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A

## TT 2020-01-15
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("month")
        .tags_filter(&["tag-1"])
        .at_date("2020-01-15")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01..")
        .expect_output("2020-W01") // Week with entries should appear
        .expect_output("2020-W03") // Week with entries should appear
        .expect_no_text("2020-W02"); // Empty week should not appear
}

#[test]
fn breakdown_by_week_should_handle_week_spanning_month_boundary() {
    // Week 1 of 2023 spans from 2023-01-02 to 2023-01-08
    // but entries might be on different sides of month boundary
    let some_content = r"## TT 2023-01-30
- #tag-1 1h Task A

## TT 2023-02-01
- #tag-1 2h Task B

## TT 2023-02-05
- #tag-1 1h Task C";

    Cmd::given()
        .breakdown_flag("week")
        .tags_filter(&["tag-1"])
        .at_date("2023-02-05")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2023-W05") // Week 5 contains Jan 30 and Feb 1-5
        .expect_output("2023-01-30 (")
        .expect_output("2023-02-01 (")
        .expect_output("2023-02-05 (")
        .expect_output("4h 00m");
}

#[test]
fn breakdown_by_month_should_handle_weeks_spanning_months() {
    // Week that spans from one month into the next
    // Week 5 of 2023 has days in both January and February
    let some_content = r"## TT 2023-01-30
- #tag-1 1h Task A

## TT 2023-02-01
- #tag-1 2h Task B";

    Cmd::given()
        .breakdown_flag("month")
        .tags_filter(&["tag-1"])
        .at_date("2023-02-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2023-01..")
        .expect_output("2023-02..")
        .expect_output("2023-W05") // Week appears in both months
        .expect_output("3h 00m");
}
