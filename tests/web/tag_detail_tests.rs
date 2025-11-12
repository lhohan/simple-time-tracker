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

#[tokio::test]
async fn tag_detail_should_handle_single_entry_for_tag() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #solo-tag 45m Single task\n",
        )
        .when_get("/api/tag/solo-tag")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("solo-tag")
        .expect_contains("Single task")
        .expect_contains("45 min");
}

#[tokio::test]
async fn tag_detail_should_show_no_entries_after_filtering() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-10\n\
             - #project-alpha 2h Old work\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/tag/project-alpha")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("No entries found");
}

#[tokio::test]
async fn tag_detail_should_handle_tag_with_zero_duration() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #quick-tag 0m Planning\n",
        )
        .when_get("/api/tag/quick-tag")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("quick-tag");
}

#[tokio::test]
async fn tag_detail_should_handle_very_large_durations() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #epic-tag 999h 59m Marathon session\n",
        )
        .when_get("/api/tag/epic-tag")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("epic-tag")
        .expect_contains("Marathon session")
        .expect_contains("59999 min");
}

#[tokio::test]
async fn tag_detail_should_aggregate_entries_across_multiple_dates() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-13\n\
             - #multi-day 2h Day 1\n\
             ## TT 2025-01-14\n\
             - #multi-day 3h Day 2\n\
             ## TT 2025-01-15\n\
             - #multi-day 1h Day 3\n",
        )
        .when_get("/api/tag/multi-day")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("Day 1")
        .expect_contains("Day 2")
        .expect_contains("Day 3");
}

#[tokio::test]
async fn tag_detail_should_show_entries_with_and_without_outcomes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha ##feature-complete 2h Task with outcome\n\
             - #project-alpha 1h Task without outcome\n",
        )
        .when_get("/api/tag/project-alpha")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("Task with outcome")
        .expect_contains("Task without outcome");
}

#[tokio::test]
async fn tag_detail_should_handle_mixed_time_formats() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #mixed-tag 2h 30m Entry 1\n\
             - #mixed-tag 1h Entry 2\n\
             - #mixed-tag 45m Entry 3\n",
        )
        .when_get("/api/tag/mixed-tag")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("150 min")
        .expect_contains("60 min")
        .expect_contains("45 min");
}

#[tokio::test]
async fn tag_detail_should_respect_this_week_filter() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-05\n\
             - #project-alpha 2h Last week\n\
             ## TT 2025-01-13\n\
             - #project-alpha 3h This week\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/tag/project-alpha")
        .with_query("period=this-week")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("This week")
        .expect_not_contains("Last week");
}

#[tokio::test]
async fn tag_detail_should_respect_this_month_filter() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2024-12-15\n\
             - #project-alpha 2h Last month\n\
             ## TT 2025-01-15\n\
             - #project-alpha 3h This month\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/tag/project-alpha")
        .with_query("period=this-month")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("This month")
        .expect_not_contains("Last month");
}
