pub mod test_helpers;
use test_helpers::*;

#[test]
fn test_basic_markdown_output() {
    CommandSpec::new()
        .with_file(
            r"
        ## TT 2020-01-01
        - #prj-1 30m
        - #prj-2  2p
        - #prj-3  20m
        - #prj-1  1h
        ",
        )
        .with_format("markdown")
        .when_run()
        .should_succeed()
        .expect_output("# Time Tracking Report")
        .expect_output("## Overview")
        .expect_output("- **prj-1**:  1h 30m (53%)")
        .expect_output("- **prj-2**:  1h 00m (35%)")
        .expect_output("- **prj-3**:  0h 20m (12%)");
}
