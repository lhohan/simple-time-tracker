use super::common::WebApp;

#[tokio::test]
async fn dashboard_should_render_with_hardcoded_data() {
    WebApp::given()
        .when_get("/")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("Time Tracker Dashboard")
        .expect_contains("8h 30m");
}

#[tokio::test]
async fn dashboard_should_show_real_time_tracking_data() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h 30m Building dashboard\n\
             - #project-beta 1h 15m Code review\n",
        )
        .when_get("/")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_contains("3h 45m");
}

#[tokio::test]
async fn dashboard_should_filter_by_this_week() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-13\n\
             - #project-alpha 2h Old entry\n\
             ## TT 2025-01-15\n\
             - #project-beta 3h Recent entry\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=this-week")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-beta")
        .expect_contains("180 min");
}

#[tokio::test]
async fn dashboard_should_filter_by_today() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #project-alpha 2h Yesterday\n\
             ## TT 2025-01-15\n\
             - #project-beta 3h Today\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-beta")
        .expect_not_contains("project-alpha");
}

#[tokio::test]
async fn dashboard_should_limit_to_top_projects_when_limit_enabled() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 10h Main project\n\
             - #project-beta 5h Secondary project\n\
             - #project-gamma 2h Minor project\n\
             - #project-delta 1h Small task\n",
        )
        .when_get("/api/dashboard")
        .with_query("limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_not_contains("project-delta");
}

#[tokio::test]
async fn dashboard_should_show_no_data_when_period_has_no_entries() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-10\n\
             - #project-alpha 2h Old work\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("No data for this period");
}

#[tokio::test]
async fn health_check_should_return_ok() {
    WebApp::given()
        .when_get("/health")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("OK");
}

#[tokio::test]
async fn dashboard_should_work_with_invalid_period_gracefully() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h Work\n",
        )
        .when_get("/api/dashboard")
        .with_query("period=not-a-valid-period")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha");
}

#[tokio::test]
async fn dashboard_should_combine_limit_and_period_filters() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #old-project 10h Yesterday work\n\
             ## TT 2025-01-15\n\
             - #project-alpha 10h Today work\n\
             - #project-beta 5h Today work\n\
             - #project-gamma 2h Today work\n\
             - #project-delta 1h Today work\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=today&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_not_contains("old-project")
        .expect_not_contains("project-delta");
}

#[tokio::test]
async fn dashboard_should_display_formatted_time_with_only_hours() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 3h Building\n",
        )
        .when_get("/")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("3h");
}

#[tokio::test]
async fn dashboard_should_display_formatted_time_with_only_minutes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 45m Quick task\n",
        )
        .when_get("/")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("45m");
}

#[tokio::test]
async fn dashboard_should_combine_period_this_week_and_limit() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-13\n\
             - #project-alpha 10h Main work\n\
             - #project-beta 5h Secondary\n\
             - #project-gamma 2h Minor\n\
             - #project-delta 1h Tiny\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=this-week&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_not_contains("project-delta");
}

#[tokio::test]
async fn dashboard_should_filter_by_this_month() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2024-12-15\n\
             - #old-project 5h Last month\n\
             ## TT 2025-01-15\n\
             - #new-project 3h This month\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=this-month")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("new-project")
        .expect_not_contains("old-project");
}

#[tokio::test]
async fn dashboard_should_filter_by_custom_date_range() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-10\n\
             - #before-range 2h Before\n\
             ## TT 2025-01-15\n\
             - #in-range 5h Within range\n\
             ## TT 2025-01-20\n\
             - #after-range 3h After\n",
        )
        .when_get("/api/dashboard")
        .with_query("from=2025-01-14&to=2025-01-16")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("in-range")
        .expect_not_contains("before-range")
        .expect_not_contains("after-range");
}

#[tokio::test]
async fn dashboard_should_handle_date_range_with_limit() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 10h Main\n\
             - #project-beta 5h Secondary\n\
             - #project-gamma 2h Minor\n\
             - #project-delta 1h Tiny\n\
             ## TT 2025-01-20\n\
             - #project-zeta 10h Outside range\n",
        )
        .when_get("/api/dashboard")
        .with_query("from=2025-01-14&to=2025-01-16&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_not_contains("project-delta")
        .expect_not_contains("project-zeta");
}

#[tokio::test]
async fn dashboard_should_handle_single_day_date_range() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #day-before 2h Before\n\
             ## TT 2025-01-15\n\
             - #target-day 5h Target\n\
             ## TT 2025-01-16\n\
             - #day-after 3h After\n",
        )
        .when_get("/api/dashboard")
        .with_query("from=2025-01-15&to=2025-01-15")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("target-day")
        .expect_not_contains("day-before")
        .expect_not_contains("day-after");
}

#[tokio::test]
async fn dashboard_should_handle_completely_empty_file() {
    WebApp::given()
        .a_file_with_content("")
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("No data");
}

#[tokio::test]
async fn dashboard_should_handle_single_project_only() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #only-project 2h 30m Solo work\n",
        )
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("only-project")
        .expect_contains("150 min");
}

#[tokio::test]
async fn dashboard_should_handle_very_large_durations() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #epic-project 999h 59m Marathon coding\n",
        )
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("epic-project")
        .expect_contains("59999 min");
}

#[tokio::test]
async fn dashboard_should_handle_zero_duration_entries() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 0m Planning\n\
             - #project-beta 2h Real work\n",
        )
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-beta");
}

#[tokio::test]
async fn dashboard_partial_should_render_empty_list() {
    WebApp::given()
        .a_file_with_content("")
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200);
}

#[tokio::test]
async fn dashboard_should_handle_mixed_hour_and_minute_formats() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h 30m Mixed format\n\
             - #project-beta 1h Only hours\n\
             - #project-gamma 45m Only minutes\n",
        )
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_contains("project-gamma");
}

#[tokio::test]
async fn dashboard_should_aggregate_multiple_entries_same_project() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 1h Task 1\n\
             - #project-alpha 2h Task 2\n\
             - #project-alpha 30m Task 3\n",
        )
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("210 min");
}

#[tokio::test]
async fn dashboard_should_handle_projects_with_special_characters() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-with-dashes 2h Work\n\
             - #project_with_underscores 1h More work\n",
        )
        .when_get("/api/dashboard")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-with-dashes")
        .expect_contains("project_with_underscores");
}
