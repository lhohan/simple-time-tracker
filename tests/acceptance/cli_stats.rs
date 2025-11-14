use crate::cli_stats_common::CliStatistics;

const SIMPLE_CONTENT: &str = r"## TT 2020-01-01
- #dev 1h Task1
- #work 2h Task2";

#[test]
fn should_record_input_flag() {
    CliStatistics::given()
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_mode("cli")
        .with_success()
        .with_flags_used(vec!["input"])
        .validate();
}

#[test]
fn should_record_verbose_flag() {
    CliStatistics::given()
        .run_with_flag("verbose")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "verbose"])
        .with_mode("cli")
        .with_success()
        .validate();
}

#[test]
fn should_record_limit_flag() {
    CliStatistics::given()
        .run_with_flag("limit")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "limit"])
        .validate();
}

#[test]
fn should_record_tags_filter() {
    CliStatistics::given()
        .run_with_filter("tags", "dev")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "tags"])
        .validate();
}

#[test]
fn should_record_project_filter() {
    CliStatistics::given()
        .run_with_filter("project", "dev")
        .run_with_flag("details")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "project", "details"])
        .validate();
}

#[test]
fn should_record_from_date_filter() {
    CliStatistics::given()
        .run_with_filter("from", "2020-01-01")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .having_all_flags(vec!["input", "from"])
        .validate();
}

#[test]
fn should_record_period_filter() {
    CliStatistics::given()
        .run_with_filter("period", "this-week")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .having_all_flags(vec!["input", "period"])
        .validate();
}

#[test]
fn should_record_breakdown_flag() {
    CliStatistics::given()
        .run_with_filter("breakdown", "day")
        .run_with_filter("tags", "dev")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .having_all_flags(vec!["input", "tags", "breakdown"])
        .validate();
}

#[test]
fn should_record_exclude_tags() {
    CliStatistics::given()
        .run_with_filter("exclude-tags", "work")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .having_all_flags(vec!["input", "exclude_tags"])
        .validate();
}

#[test]
fn should_record_format_option() {
    CliStatistics::given()
        .run_with_filter("format", "markdown")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .having_all_flags(vec!["input", "format"])
        .validate();
}

#[test]
fn should_record_verbose_with_tags_combination() {
    CliStatistics::given()
        .run_with_flag("verbose")
        .run_with_filter("tags", "dev")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "verbose", "tags"])
        .validate();
}

#[test]
fn should_record_tags_with_breakdown_combination() {
    CliStatistics::given()
        .run_with_filter("tags", "dev")
        .run_with_filter("breakdown", "week")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "tags", "breakdown"])
        .validate();
}

#[test]
fn should_record_project_with_from_date_combination() {
    CliStatistics::given()
        .run_with_filter("project", "dev")
        .run_with_filter("from", "2020-01-01")
        .run_with_flag("details")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .having_all_flags(vec!["input", "project", "from", "details"])
        .validate();
}

#[test]
fn should_record_multiple_filters_combination() {
    CliStatistics::given()
        .run_with_flag("verbose")
        .run_with_flag("limit")
        .run_with_filter("tags", "dev")
        .run_with_filter("breakdown", "day")
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_flags_used(vec!["input", "verbose", "limit", "tags", "breakdown"])
        .validate();
}

#[test]
fn should_not_record_format_when_using_default() {
    CliStatistics::given()
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .not_having_flag("format")
        .validate();
}

#[test]
fn should_not_record_web_flag_in_cli_mode() {
    CliStatistics::given()
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_mode("cli")
        .not_having_flag("web")
        .not_having_flag("port")
        .not_having_flag("host")
        .validate();
}

#[test]
fn should_have_timestamp_in_record() {
    CliStatistics::given()
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .validate();
}

#[test]
fn should_create_stats_file_in_stats_directory() {
    CliStatistics::given()
        .run_with_file(SIMPLE_CONTENT)
        .when_executed()
        .should_succeed()
        .should_record_stats()
        .with_success()
        .validate();
}
