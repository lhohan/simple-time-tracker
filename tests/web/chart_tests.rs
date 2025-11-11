use crate::common::WebApp;

#[tokio::test]
async fn bar_chart_should_render_with_project_data() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 3h Building features
- #project-beta 2h Code review
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-bar")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("<canvas")
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_contains("Chart");
}

#[tokio::test]
async fn bar_chart_should_respect_period_filter() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 3h Today's work

## TT 2025-01-10
- #project-beta 2h Last week's work
"#;

    WebApp::given()
        .a_file_with_content(input)
        .at_date("2025-01-15")
        .when_get("/api/chart/projects-bar")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_not_contains("project-beta");
}

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
async fn bar_chart_should_respect_limit_filter() {
    let input = r#"
## TT 2025-01-15
- #project-alpha 10h Main work
- #project-beta 5h Secondary work
- #project-gamma 2h Minor work
- #project-delta 1h Small task
"#;

    WebApp::given()
        .a_file_with_content(input)
        .when_get("/api/chart/projects-bar")
        .with_query("limit=true")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_contains("project-beta")
        .expect_not_contains("project-delta");
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
