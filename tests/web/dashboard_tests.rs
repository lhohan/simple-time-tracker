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
