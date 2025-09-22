use crate::common::*;

#[test]
fn app_should_use_first_tag_as_main_project() {
    let content = r"## TT 2020-01-01
- #tag1 #tag2 #tag3 1h Task A";

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_project("tag1")
        .validate();
}

mod project_filter {
    use crate::common::*;
    use rstest::rstest;

    #[rstest]
    fn project_filter_should_match_any_tag_in_entry(#[values("tag1", "tag2", "tag3")] tag: &str) {
        let content = r"## TT 2024-01-15
- #tag1 #tag2 #tag3 1h Task A";

        Cmd::given()
            .project_filter(tag)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_task("Task A");
    }
}

mod tags_filter {
    use crate::common::*;

    #[test]
    fn app_should_support_tags_filter() {
        let content = r"## TT 2020-01-01
- #tag1 1h Task A";

        Cmd::given()
            .tags_filter(&["tag1"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed();
    }

    #[test]
    fn tags_filter_should_show_matching_entries() {
        let content = r"## TT 2024-01-15
- #tag-1 1h Task A
- #tag-2 2h Task B";

        Cmd::given()
            .tags_filter(&["tag-1"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_project("tag-1")
            .validate()
            .expect_no_text("tag-2");
    }

    #[test]
    fn tags_filter_should_show_all_entries_when_empty() {
        let content = r"## TT 2024-01-15
- #tag-1 1h Task A
";

        Cmd::given()
            .tags_filter(&[])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_project("tag-1")
            .validate();
    }

    #[test]
    fn tags_filter_should_show_no_data_when_no_match() {
        let content = r"## TT 2024-01-15
- #tag-1 1h Task A
";

        Cmd::given()
            .tags_filter(&["tag-2"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_no_data_found();
    }

    #[test]
    fn tags_filter_should_show_entries_matching_any_tag() {
        let content = r"## TT 2024-01-15
- #tag-1 1h Task A
- #tag-2 2h Task B
- #tag-3 4h Task C";

        Cmd::given()
            .tags_filter(&["tag-1", "tag-2"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_project("tag-1")
            .validate()
            .expect_project("tag-2")
            .validate()
            .expect_no_text("tag-3");
    }
}

mod exclude_tags_filter {
    use crate::common::*;

    #[test]
    fn exclude_tags_filter_should_hide_matching_entries() {
        let content = r"## TT 2020-01-01
- #tag1 1h Task A
- #tag2 1h Task B";

        Cmd::given()
            .exclude_tags_filter(&["tag2"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_no_text("tag2")
            .expect_project("tag1")
            .validate();
    }

    #[test]
    fn exclude_tags_filter_should_hide_entries_matching_any_tag() {
        let content = r"## TT 2020-01-01
- #tag1 1h Task A
- #tag2 1h Task B
- #tag3 1h Task C";

        Cmd::given()
            .exclude_tags_filter(&["tag2", "tag3"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_no_text("tag2")
            .expect_no_text("tag3")
            .expect_project("tag1")
            .taking("1h")
            .validate();
    }
    #[test]
    fn exclude_tags_filter_should_hide_entry_if_any_tag_matches() {
        let content = r"## TT 2020-01-01
- #tag1 #tag2 1h Task A
- #tag1 1h Task B";

        Cmd::given()
            .exclude_tags_filter(&["tag2"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_no_text("tag2")
            .expect_project("tag1")
            .taking("1h")
            .validate();
    }

    #[test]
    fn app_should_support_exclude_tags_filter() {
        let content = r"## TT 2020-01-01
- #tag1 1h Task A";

        Cmd::given()
            .exclude_tags_filter(&["tag1"])
            .a_file_with_content(content)
            .when_run()
            .should_succeed();
    }
}
