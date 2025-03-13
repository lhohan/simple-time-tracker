pub mod test_helpers;
use test_helpers::*;

#[test]
fn first_tag_is_main_context() {
    let content = r#"## TT 2020-01-01
- #tag1 #tag2 #tag3 1h Task A"#;

    CommandSpec::new()
        .with_file(content)
        .when_run()
        .should_succeed()
        .expect_project("tag1")
        .validate();
}

mod filter_tags {
    use crate::test_helpers::CommandSpec;

    #[ignore]
    #[test]
    fn project_filter_works_on_all_tags() {
        let content = r#"## TT 2024-01-15
- #tag1 #tag2 1h Task A"#;

        CommandSpec::new()
            .with_file(content)
            .with_project_filter("tag2")
            .when_run()
            .should_succeed()
            .expect_project("tag2")
            .validate();
    }

    #[ignore]
    #[test]
    fn project_filter_shows_related_tags() {
        let content = r#"## TT 2024-01-15
- #tag1 #tag2 #tag3 1h Task A"#;

        CommandSpec::new()
            .with_file(content)
            .with_project_filter("prj-timetracker")
            .when_run()
            .should_succeed()
            .expect_project("tag1")
            .with_context("tag2, tag3")
            .validate();
    }
}

mod exclude_tags {
    use crate::test_helpers::CommandSpec;

    #[test]
    fn excluded_tags_are_filtered() {
        let content = r#"## TT 2020-01-01
- #tag1 1h Task A
- #tag2 1h Task B"#;

        CommandSpec::new()
            .with_file(content)
            .with_exclude_tags_filter(&["tag2"])
            .when_run()
            .should_succeed()
            .expect_no_text("tag2")
            .expect_project("tag1")
            .validate();
    }

    #[test]
    fn excluded_tags_are_filtered_multiple_tags_excluded() {
        let content = r#"## TT 2020-01-01
- #tag1 1h Task A
- #tag2 1h Task B
- #tag3 1h Task C"#;

        CommandSpec::new()
            .with_file(content)
            .with_exclude_tags_filter(&["tag2", "tag3"])
            .when_run()
            .should_succeed()
            .expect_no_text("tag2")
            .expect_no_text("tag3")
            .expect_project("tag1")
            .taking("1h")
            .validate();
    }
    #[test]
    fn excluded_tags_are_filtered_entry_with_multiple_tags() {
        let content = r#"## TT 2020-01-01
- #tag1 #tag2 1h Task A
- #tag1 1h Task B"#;

        CommandSpec::new()
            .with_file(content)
            .with_exclude_tags_filter(&["tag2"])
            .when_run()
            .should_succeed()
            .expect_no_text("tag2")
            .expect_project("tag1")
            .taking("1h")
            .validate();
    }

    #[test]
    fn exclude_tags_filter_should_work() {
        let content = r#"## TT 2020-01-01
- #tag1 1h Task A"#;

        CommandSpec::new()
            .with_file(content)
            .with_exclude_tags_filter(&["tag1"])
            .when_run()
            .should_succeed();
    }
}
