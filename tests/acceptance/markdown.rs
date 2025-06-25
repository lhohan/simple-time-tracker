use crate::common::Cmd;

#[test]
fn output_should_format_as_markdown_when_format_specified() {
    Cmd::given()
        .output_format("markdown")
        .a_file_with_content(
            r"
            ## TT 2020-01-01
            - #prj-1 1h
            - #prj-2 2h
            - #prj-3 4h
            ",
        )
        .when_run()
        .should_succeed()
        .expect_output("# Time Tracking Report")
        .expect_output("- **prj-1**:  1h 00m (14%)")
        .expect_output("- **prj-2**:  2h 00m (29%)")
        .expect_output("- **prj-3**:  4h 00m (57%)");
}
