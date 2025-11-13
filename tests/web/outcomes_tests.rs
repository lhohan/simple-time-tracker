use super::common::WebApp;

#[tokio::test]
async fn outcomes_page_should_render_basic_outcomes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha ##feature-implementation 2h 30m Building feature\n\
             - #project-beta ##bug-fix 1h 15m Fixing bugs\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("feature-implementation")
        .expect_contains("bug-fix")
        .expect_contains("3h 45m");
}

#[tokio::test]
async fn outcomes_page_should_show_no_data_message_when_empty() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h Work without outcomes\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("No outcomes data available");
}

#[tokio::test]
async fn outcomes_page_should_aggregate_same_outcomes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha ##feature-work 2h Task A\n\
             - #project-beta ##feature-work 3h Task B\n\
             - #project-gamma ##feature-work 1h Task C\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("feature-work")
        .expect_contains("6h");
}

#[tokio::test]
async fn outcomes_page_should_handle_single_outcome() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha ##single-outcome 45m Only task\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("single-outcome")
        .expect_contains("45m");
}

#[tokio::test]
async fn outcomes_page_should_handle_zero_duration_outcomes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha ##planning 0m Quick planning\n\
             - #project-beta ##implementation 2h Real work\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("implementation");
}

#[tokio::test]
async fn outcomes_partial_should_filter_by_today() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #project-alpha ##old-outcome 2h Yesterday\n\
             ## TT 2025-01-15\n\
             - #project-beta ##today-outcome 3h Today\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/outcomes")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("today-outcome")
        .expect_not_contains("old-outcome");
}

#[tokio::test]
async fn outcomes_partial_should_filter_by_this_week() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-13\n\
             - #project-alpha ##this-week-outcome 2h This week\n\
             ## TT 2025-01-05\n\
             - #project-beta ##last-week-outcome 3h Last week\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/outcomes")
        .with_query("period=this-week")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("this-week-outcome")
        .expect_not_contains("last-week-outcome");
}

#[tokio::test]
async fn outcomes_partial_should_filter_by_this_month() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha ##this-month-outcome 2h This month\n\
             ## TT 2024-12-15\n\
             - #project-beta ##last-month-outcome 3h Last month\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/outcomes")
        .with_query("period=this-month")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("this-month-outcome")
        .expect_not_contains("last-month-outcome");
}

#[tokio::test]
async fn outcomes_partial_should_filter_by_custom_date_range() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-10\n\
             - #before ##before-range 2h Before\n\
             ## TT 2025-01-15\n\
             - #during ##in-range 5h Within range\n\
             ## TT 2025-01-20\n\
             - #after ##after-range 3h After\n",
        )
        .when_get("/api/outcomes")
        .with_query("from=2025-01-14&to=2025-01-16")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("in-range")
        .expect_not_contains("before-range")
        .expect_not_contains("after-range");
}

#[tokio::test]
async fn outcomes_partial_should_limit_to_top_outcomes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #proj ##outcome-alpha 10h Main outcome\n\
             - #proj ##outcome-beta 5h Secondary outcome\n\
             - #proj ##outcome-gamma 2h Minor outcome\n\
             - #proj ##outcome-delta 1h Tiny outcome\n",
        )
        .when_get("/api/outcomes")
        .with_query("limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("outcome-alpha")
        .expect_contains("outcome-beta")
        .expect_not_contains("outcome-delta");
}

#[tokio::test]
async fn outcomes_partial_should_combine_period_and_limit_filters() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #proj ##old-outcome 10h Yesterday\n\
             ## TT 2025-01-15\n\
             - #proj ##outcome-alpha 10h Main today\n\
             - #proj ##outcome-beta 5h Secondary today\n\
             - #proj ##outcome-gamma 2h Minor today\n\
             - #proj ##outcome-delta 1h Tiny today\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/outcomes")
        .with_query("period=today&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("outcome-alpha")
        .expect_not_contains("old-outcome");
}

#[tokio::test]
async fn outcomes_partial_should_handle_empty_results_after_filtering() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-10\n\
             - #proj ##old-outcome 2h Old work\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/outcomes")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200);
}

#[tokio::test]
async fn outcomes_chart_should_render_pie_chart_data() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #proj ##outcome-alpha 3h Work A\n\
             - #proj ##outcome-beta 2h Work B\n",
        )
        .when_get("/api/chart/outcomes-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("<canvas")
        .expect_contains("outcome-alpha")
        .expect_contains("outcome-beta");
}

#[tokio::test]
async fn outcomes_chart_should_respect_period_filter() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #proj ##old-outcome 5h Yesterday\n\
             ## TT 2025-01-15\n\
             - #proj ##today-outcome 3h Today\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/chart/outcomes-pie")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("today-outcome")
        .expect_not_contains("old-outcome");
}

#[tokio::test]
async fn outcomes_chart_should_respect_limit_filter() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #proj ##outcome-alpha 10h Main\n\
             - #proj ##outcome-beta 5h Secondary\n\
             - #proj ##outcome-gamma 2h Minor\n\
             - #proj ##outcome-delta 1h Tiny\n",
        )
        .when_get("/api/chart/outcomes-pie")
        .with_query("limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("outcome-alpha")
        .expect_not_contains("outcome-delta");
}

#[tokio::test]
async fn outcomes_chart_should_combine_filters() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #proj ##old-outcome 10h Yesterday\n\
             ## TT 2025-01-15\n\
             - #proj ##outcome-alpha 10h Today main\n\
             - #proj ##outcome-beta 5h Today secondary\n\
             - #proj ##outcome-gamma 1h Today minor\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/chart/outcomes-pie")
        .with_query("period=today&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("outcome-alpha")
        .expect_not_contains("old-outcome");
}

#[tokio::test]
async fn outcomes_chart_should_handle_empty_data() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #proj 2h Work without outcomes\n",
        )
        .when_get("/api/chart/outcomes-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("<canvas");
}

#[tokio::test]
async fn outcomes_partial_should_handle_single_day_date_range() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-14\n\
             - #proj ##day-before 2h Before\n\
             ## TT 2025-01-15\n\
             - #proj ##target-day 5h Target\n\
             ## TT 2025-01-16\n\
             - #proj ##day-after 3h After\n",
        )
        .when_get("/api/outcomes")
        .with_query("from=2025-01-15&to=2025-01-15")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("target-day")
        .expect_not_contains("day-before")
        .expect_not_contains("day-after");
}

#[tokio::test]
async fn outcomes_page_should_handle_mixed_entries_with_and_without_outcomes() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 1h No outcome\n\
             - #project-beta ##with-outcome 2h Has outcome\n\
             - #project-gamma 30m Also no outcome\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("with-outcome");
}

#[tokio::test]
async fn outcomes_page_should_handle_very_large_durations() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #proj ##large-outcome 999h Epic task\n",
        )
        .when_get("/outcomes")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("large-outcome")
        .expect_contains("999h");
}

#[tokio::test]
async fn outcomes_partial_should_handle_date_range_with_limit() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #proj ##outcome-alpha 10h Main\n\
             - #proj ##outcome-beta 5h Secondary\n\
             - #proj ##outcome-gamma 2h Minor\n\
             - #proj ##outcome-delta 1h Tiny\n\
             ## TT 2025-01-20\n\
             - #proj ##outcome-zeta 10h Outside range\n",
        )
        .when_get("/api/outcomes")
        .with_query("from=2025-01-14&to=2025-01-16&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("outcome-alpha")
        .expect_contains("outcome-beta")
        .expect_not_contains("outcome-zeta");
}
