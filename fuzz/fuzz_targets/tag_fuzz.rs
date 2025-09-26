#![no_main]

use assert_cmd::Command;
use assert_fs::prelude::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(fuzzed_tag) = std::str::from_utf8(data) {
        let time_entry = format!("- #{} 1h Task description", fuzzed_tag);
        let content = format!("## TT 2020-01-01\n{}", time_entry);

        if let Ok(temp) = assert_fs::TempDir::new() {
            let input_file = temp.child("fuzz.md");
            if input_file.write_str(&content).is_ok() {
                if let Ok(mut cmd) = Command::cargo_bin("tt") {
                    cmd.arg("--input").arg(input_file.path());
                    let _ = cmd.output();
                }
            }
        }
    }
});
