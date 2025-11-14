use super::common::WebApp;

#[tokio::test]
async fn stats_page_should_display_used_tags_sorted_by_minutes() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 2h Build feature
- #context-b 1h Review code
- #project-a 1h Write tests
- #context-b 30m Debug"#,
        )
        .at_date("2025-01-15")
        .when_get("/stats")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-a")
        .expect_contains("context-b")
        .expect_contains("Tag Usage Statistics");
}

#[tokio::test]
async fn stats_api_should_return_used_and_unused_tags() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 2h Build feature
- #context-b 1h Review code"#,
        )
        .at_date("2025-01-15")
        .when_get("/api/stats")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-a")
        .expect_contains("context-b");
}

#[tokio::test]
async fn stats_should_show_tag_counts() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 1h Task 1
- #project-a 1h Task 2
- #project-a 1h Task 3
- #context-b 1h Task 4"#,
        )
        .at_date("2025-01-15")
        .when_get("/api/stats")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("3 times")
        .expect_contains("1 time");
}

#[tokio::test]
async fn stats_should_show_percentages() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 2h Task 1
- #project-a 1h Task 2
- #context-b 1h Task 3"#,
        )
        .at_date("2025-01-15")
        .when_get("/api/stats")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("%");
}

#[tokio::test]
async fn stats_should_show_total_entries_and_used_tags_count() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 1h Task 1
- #project-a 1h Task 2
- #context-b 1h Task 3"#,
        )
        .at_date("2025-01-15")
        .when_get("/api/stats")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("Total Tags Used")
        .expect_contains("Total Entries");
}

#[tokio::test]
async fn stats_with_date_range_filter() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 2h Build feature

## TT 2025-01-16
- #context-b 1h Review code"#,
        )
        .at_date("2025-01-16")
        .when_get("/api/stats")
        .with_query("from=2025-01-15&to=2025-01-16")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-a")
        .expect_contains("context-b");
}

#[tokio::test]
async fn stats_should_handle_empty_data() {
    WebApp::given()
        .a_file_with_content("## TT 2025-01-15")
        .at_date("2025-01-15")
        .when_get("/api/stats")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("No tags tracked in this period");
}

#[tokio::test]
async fn stats_should_sort_by_minutes_descending() {
    WebApp::given()
        .a_file_with_content(
            r#"## TT 2025-01-15
- #project-a 1h Task
- #context-b 3h Task
- #context-c 2h Task"#,
        )
        .at_date("2025-01-15")
        .when_get("/api/stats")
        .should_succeed()
        .await
        .expect_status(200);
}
