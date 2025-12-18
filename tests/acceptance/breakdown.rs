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
fn breakdown_with_details_should_require_tags() {
    let some_content = r"## TT 2020-01-01
- #tag-1 1h Task A";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .a_file_with_content(some_content)
        .when_run()
        .should_fail()
        .expect_error("--details flag requires");
}

#[test]
fn breakdown_day_with_details_should_show_tasks_per_day() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A
- #project-b 2h Task B";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .tags_filter(&["project-a", "project-b"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01-01 (")
        .expect_output("3h 00m")
        .expect_task_with_duration("project-a", "1h 00m")
        .expect_task_with_duration("project-b", "2h 00m");
}

#[test]
fn breakdown_day_with_details_should_show_tasks_grouped_by_day() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A

## TT 2020-01-02
- #project-b 2h Task B";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .tags_filter(&["project-a", "project-b"])
        .at_date("2020-01-02")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01-01 (")
        .expect_output("2020-01-02 (")
        .expect_task_with_duration("project-a", "1h 00m")
        .expect_task_with_duration("project-b", "2h 00m")
        .expect_output_pattern(r"2020-01-01 \((?s).*project-a(?s).*2020-01-02 \((?s).*project-b");
}

#[test]
fn breakdown_day_with_details_should_aggregate_same_task_descriptions() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A
- #project-a 30m Task B";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .tags_filter(&["project-a"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01-01 (")
        .expect_output("1h 30m")
        .expect_task_with_duration("project-a", "1h 30m");
}

#[test]
fn breakdown_week_with_details_should_show_tasks_per_day() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A

## TT 2020-01-02
- #project-b 2h Task B";

    Cmd::given()
        .breakdown_flag("week")
        .details_flag()
        .tags_filter(&["project-a", "project-b"])
        .at_date("2020-01-02")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-W01")
        .expect_output("2020-01-01 (")
        .expect_output("2020-01-02 (")
        .expect_task_with_duration("project-a", "1h 00m")
        .expect_task_with_duration("project-b", "2h 00m");
}

#[test]
fn breakdown_month_with_details_should_show_tasks_per_week() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A

## TT 2020-01-08
- #project-b 2h Task B";

    Cmd::given()
        .breakdown_flag("month")
        .details_flag()
        .tags_filter(&["project-a", "project-b"])
        .at_date("2020-01-08")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01")
        .expect_output("2020-W01")
        .expect_output("2020-W02")
        .expect_task_with_duration("project-a", "1h 00m")
        .expect_task_with_duration("project-b", "2h 00m");
}

#[test]
fn breakdown_year_with_details_should_show_tasks_per_month() {
    let some_content = r"## TT 2020-01-15
- #project-a 1h Task A

## TT 2020-02-20
- #project-b 2h Task B";

    Cmd::given()
        .breakdown_flag("year")
        .details_flag()
        .tags_filter(&["project-a", "project-b"])
        .at_date("2020-02-20")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020")
        .expect_output("2020-01")
        .expect_output("2020-02")
        .expect_task_with_duration("project-a", "1h 00m")
        .expect_task_with_duration("project-b", "2h 00m");
}

#[test]
fn breakdown_with_details_should_show_no_tag_placeholder() {
    let some_content = r"## TT 2020-01-01
- 1h Task without tag";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .tags_filter(&["work"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("No data found.");
}

#[test]
fn breakdown_with_details_should_show_entries_with_and_without_tags() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A
- 30m Task without tag";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .tags_filter(&["project-a"])
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("2020-01-01 (")
        .expect_output("1h 00m")
        .expect_task_with_duration("project-a", "1h 00m");
}

#[test]
fn breakdown_with_details_and_markdown_format_should_show_markdown_output() {
    let some_content = r"## TT 2020-01-01
- #project-a 1h Task A
- #project-b 2h Task B";

    Cmd::given()
        .breakdown_flag("day")
        .details_flag()
        .tags_filter(&["project-a", "project-b"])
        .output_format("markdown")
        .at_date("2020-01-01")
        .a_file_with_content(some_content)
        .when_run()
        .should_succeed()
        .expect_output("# Time Breakdown Report")
        .expect_output("2020-01-01")
        .expect_output("project-a")
        .expect_output("project-b")
        .expect_output("1h 00m")
        .expect_output("2h 00m");
}
