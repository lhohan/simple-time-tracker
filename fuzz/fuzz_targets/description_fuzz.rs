#![no_main]

use libfuzzer_sys::fuzz_target;
use assert_cmd::Command;
use assert_fs::prelude::*;

fuzz_target!(|data: &[u8]| {
    // Only proceed if we can convert to valid UTF-8
    if let Ok(fuzzed_description) = std::str::from_utf8(data) {
        // Create time entry with fuzzed description
        // Format: - #dev 1h {FUZZED_DESCRIPTION}
        let time_entry = format!("- #dev 1h {}", fuzzed_description);
        let content = format!("## TT 2020-01-01\n{}", time_entry);

        // Use assert_fs::TempDir like existing acceptance tests
        if let Ok(temp) = assert_fs::TempDir::new() {
            let input_file = temp.child("fuzz.md");
            if input_file.write_str(&content).is_ok() {
                // Use Command::cargo_bin like existing acceptance tests
                if let Ok(mut cmd) = Command::cargo_bin("tt") {
                    cmd.arg("--input").arg(input_file.path());

                    // Execute command and collect any crashes
                    // Don't assert on output - we want to discover panics
                    let _ = cmd.output();
                }
            }
        }
    }
});
