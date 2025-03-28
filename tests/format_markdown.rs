pub mod test_helpers;
use test_helpers::*;

#[test]
fn test_basic_markdown_output() {
    CommandSpec::new()
        .with_file(
            r"
        ## TT 2020-01-01
        - #prj-1 1h
        - #prj-2 2h
        - #prj-3 4h
        ",
        )
        .with_format("markdown")
        .when_run()
        .should_succeed()
        .expect_output("# Time Tracking Report")
        .expect_output("- **prj-1**:  1h 00m (14%)")
        .expect_output("- **prj-2**:  2h 00m (29%)")
        .expect_output("- **prj-3**:  4h 00m (57%)");
}
