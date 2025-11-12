use crate::common::WebApp;

#[tokio::test]
async fn pie_chart_should_render_with_project_data() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 3h Building features
- #project-beta 2h Code review
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("<canvas")
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_contains("pie");
}

#[tokio::test]
async fn pie_chart_should_respect_period_filter() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 3h Today's work

## TT 2025-01-10
- #project-beta 2h Last week's work
"#;

    WebApp::given()
        .a_file_with_content(input)
        .at_date("2025-01-15")
        .when_get("/api/chart/projects-pie")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_not_contains("project-beta");
}

#[tokio::test]
async fn pie_chart_should_respect_limit_filter() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 10h Main work
- #project-beta 5h Secondary work
- #project-gamma 2h Minor work
- #project-delta 1h Small task
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .with_query("limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_not_contains("project-delta");
}

#[tokio::test]
async fn pie_chart_should_combine_period_and_limit_filters() {
    let input = r#"
## TT 2025-01-14
- #old-project 10h Yesterday
## TT 2025-01-15
- #project-alpha 10h Today work
- #project-beta 5h Today work
- #project-gamma 2h Today work
- #project-delta 1h Today work
"#;

    WebApp::given()
        .a_file_with_content(input)
        .at_date("2025-01-15")
        .when_get("/api/chart/projects-pie")
        .with_query("period=today&limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_not_contains("old-project")
        .expect_not_contains("project-delta");
}

#[tokio::test]
async fn pie_chart_should_show_empty_when_no_data() {
    let input = "## TT 2025-01-15\n\n";

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("canvas");
}

#[tokio::test]
async fn pie_chart_should_filter_by_this_week() {
    let input = r#"
## TT 2025-01-10
- #old-work 5h Last week
## TT 2025-01-15
- #this-week-work 3h This week
"#;

    WebApp::given()
        .a_file_with_content(input)
        .at_date("2025-01-15")
        .when_get("/api/chart/projects-pie")
        .with_query("period=this-week")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("this-week-work")
        .expect_not_contains("old-work");
}

#[tokio::test]
async fn pie_chart_should_handle_single_data_point() {
    let input = r#"
## TT 2025-01-15
- #solo-project 2h 30m Only entry
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("<canvas")
        .expect_contains("solo-project");
}

#[tokio::test]
async fn pie_chart_should_handle_many_data_points() {
    let input = r#"
## TT 2025-01-15
- #project-01 1h Work
- #project-02 1h Work
- #project-03 1h Work
- #project-04 1h Work
- #project-05 1h Work
- #project-06 1h Work
- #project-07 1h Work
- #project-08 1h Work
- #project-09 1h Work
- #project-10 1h Work
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-01")
        .expect_contains("project-10");
}

#[tokio::test]
async fn pie_chart_should_handle_very_large_durations() {
    let input = r#"
## TT 2025-01-15
- #epic-project 999h 59m Marathon session
- #normal-project 2h Regular work
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("epic-project")
        .expect_contains("normal-project");
}

#[tokio::test]
async fn pie_chart_should_filter_by_custom_date_range() {
    let input = r#"
## TT 2025-01-10
- #before-range 2h Before
## TT 2025-01-15
- #in-range 5h Within range
## TT 2025-01-20
- #after-range 3h After
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .with_query("from=2025-01-14&to=2025-01-16")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("in-range")
        .expect_not_contains("before-range")
        .expect_not_contains("after-range");
}

#[tokio::test]
async fn pie_chart_should_combine_date_range_and_limit() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 10h Main
- #project-beta 5h Secondary
- #project-gamma 2h Minor
- #project-delta 1h Tiny
## TT 2025-01-20
- #project-zeta 10h Outside range
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
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
async fn pie_chart_should_handle_zero_duration_entries() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 0m Planning
- #project-beta 2h Real work
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-pie")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-beta");
}
