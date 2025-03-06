pub mod test_helpers;
use rstest::rstest;
use test_helpers::*;

#[rstest]
fn this_week_report(#[values("this-week", "tw")] this_week_value: &str) {
    let content = r"## TT 2020-01-01
    - #dev 1h Task1
    ## TT 2020-01-02
    - #dev 2h Task2
    ## TT 2020-01-20
    - #dev 1h Task3";

    CommandSpec::new()
        .with_file(content)
        .at_date("2020-01-01") // Testing as if we're running on Jan 1st
        .with_period(this_week_value)
        .when_run()
        .should_succeed()
        .expect_output("Week 1, 2020")
        .expect_project("dev")
        .taking("3h 00m") // Only tasks from Jan 1-2
        .validate();
}

#[rstest]
fn last_week_report(#[values("last-week", "lw")] last_week_value: &str) {
    let content = r"## TT 2020-01-01
    - #dev 1h Task1
    ## TT 2020-01-02
    - #dev 2h Task2
    ## TT 2020-01-08
    - #dev 1h Task3";

    CommandSpec::new()
        .with_file(content)
        .at_date("2020-01-08") // Testing as if we're running on Jan 8th
        .with_period(last_week_value)
        .when_run()
        .should_succeed()
        .expect_output("Week 1, 2020")
        .expect_project("dev")
        .taking("3h 00m") // Only tasks from Jan 1-2 (last week)
        .validate();
}

#[rstest]
fn last_month_report(#[values("last-month", "lm")] last_month_value: &str) {
    let content = r"## TT 2020-01-01
    - #dev 1h Task1
    ## TT 2020-01-31
    - #dev 2h Task2
    ## TT 2020-02-01
    - #dev 1h Task3";

    CommandSpec::new()
        .with_file(content)
        .at_date("2020-02-01")
        .with_period(last_month_value)
        .when_run()
        .should_succeed()
        .expect_output("2020-01")
        .expect_project("dev")
        .taking("3h 00m") // Only tasks from Jan (last month)
        .validate();
}

#[rstest]
fn this_month_report(#[values("this-month", "tm")] this_month_value: &str) {
    let content = r"## TT 2020-01-01
    - #dev 1h Task1
    ## TT 2020-01-31
    - #dev 2h Task2
    ## TT 2020-02-01
    - #dev 1h Task3";

    CommandSpec::new()
        .with_file(content)
        .at_date("2020-01-01")
        .with_period(this_month_value)
        .when_run()
        .should_succeed()
        .expect_output("2020-01")
        .expect_project("dev")
        .taking("3h 00m") // Only tasks from Jan (this month)
        .validate();
}

#[rstest]
fn today_report(
    #[values("today", "t")] value: &str,
    #[values(
        ("2020-01-01", "1h 00m"),
        ("2020-01-02", "2h 00m"),
        ("2020-01-03", "4h 00m"),
    )]
    at_date_and_expected_duration: (&str, &str),
) {
    let content = r"## TT 2020-01-01
    - #dev 1h Task1
    ## TT 2020-01-02
    - #dev 2h Task2
    ## TT 2020-01-03
    - #dev 4h Task3";

    let at_date = at_date_and_expected_duration.0;

    let expected_output = format!("today ({})", at_date);
    let expected_duration = at_date_and_expected_duration.1;

    CommandSpec::new()
        .with_file(content)
        .at_date(at_date)
        .with_period(value)
        .when_run()
        .should_succeed()
        .expect_output(&expected_output)
        .expect_project("dev")
        .taking(expected_duration) // Only tasks of 'today'
        .validate();
}
