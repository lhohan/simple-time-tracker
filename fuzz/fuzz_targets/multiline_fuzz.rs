#![no_main]

use assert_cmd::Command;
use assert_fs::prelude::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Only proceed if we can convert to valid UTF-8
    if let Ok(fuzzed_content) = std::str::from_utf8(data) {
        let lines: Vec<&str> = fuzzed_content.lines().collect();
        let mut entries = Vec::new();

        for (_, tag) in lines.iter().take(5).enumerate() {
            let entry = format!("- #{} 1h Task description", tag);
            entries.push(entry);
        }

        if entries.is_empty() {
            entries.push(format!("- #dev 1h {}", fuzzed_content));
        }

        let content = format!("## TT 2020-01-01\n{}", entries.join("\n"));

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
