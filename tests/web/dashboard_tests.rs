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
