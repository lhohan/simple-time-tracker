use super::common::WebApp;

#[tokio::test]
async fn tag_detail_should_show_individual_entries() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h 30m Building dashboard\n\
             - #project-alpha 1h 15m Code review\n\
             - #project-beta 1h Testing\n",
        )
        .when_get("/api/tag/project-alpha")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("Building dashboard")
        .expect_contains("Code review")
        .expect_contains("150 min")
        .expect_contains("75 min")
        .expect_not_contains("project-beta");
}
