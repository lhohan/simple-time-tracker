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

#[tokio::test]
async fn tag_detail_should_show_no_entries_for_nonexistent_tag() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h Work\n",
        )
        .when_get("/api/tag/nonexistent-tag")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("No entries found");
}

#[tokio::test]
async fn tag_detail_should_accept_tags_with_dashes_and_underscores() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project_alpha-beta 2h Work with valid tag chars\n",
        )
        .when_get("/api/tag/project_alpha-beta")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project_alpha-beta")
        .expect_contains("Work with valid tag chars");
}

#[tokio::test]
async fn tag_detail_should_respect_period_filter() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #project-alpha 2h Yesterday work\n\
             ## TT 2025-01-15\n\
             - #project-alpha 3h Today work\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/tag/project-alpha")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("Today work")
        .expect_contains("180 min")
        .expect_not_contains("Yesterday work");
}

#[tokio::test]
async fn tag_detail_should_respect_custom_date_range() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-10\n\
             - #project-alpha 2h Older work\n\
             ## TT 2025-01-15\n\
             - #project-alpha 3h Recent work\n\
             ## TT 2025-01-20\n\
             - #project-alpha 4h Very recent work\n",
        )
        .when_get("/api/tag/project-alpha")
        .with_query("from=2025-01-15&to=2025-01-20")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("Recent work")
        .expect_contains("Very recent work")
        .expect_contains("180 min")
        .expect_contains("240 min")
        .expect_not_contains("Older work");
}
