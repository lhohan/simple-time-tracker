pub mod test_helpers;
use test_helpers::*;

#[test]
fn project_can_be_in_any_tag_position() {
    let content = r#"## TT 2000-01-01
- #tag1 #prj-the-project #tag2 #tag3 1h Working on tests"#;

    CommandSpec::new()
        .with_file(content)
        .with_project_filter("prj-the-project")
        .when_run()
        .should_succeed()
        .expect_output("Project: prj-the-project");
}

#[ignore]
#[test]
fn shows_context_tags_in_project_details() {
    let content = r#"## TT 2024-01-15
- #rust #prj-timetracker #cli #testing 1h Working on tests"#;

    CommandSpec::new()
        .with_file(content)
        .with_project_filter("prj-timetracker")
        .when_run()
        .should_succeed()
        .expect_project("prj-timetracker")
        .with_tag("rust, cli, testing")
        .validate();
}
