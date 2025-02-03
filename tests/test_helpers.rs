use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::sync::Arc;

#[derive(Default)]
struct CommandArgs {
    args: Vec<String>,
}

impl CommandArgs {
    fn new() -> Self {
        Self::default()
    }

    fn add_flag(&mut self, flag: &str) {
        self.args.push(format!("--{}", flag));
    }

    fn add_option(&mut self, option: &str, value: &str) {
        self.args.push(format!("--{}", option));
        self.args.push(value.to_string());
    }

    fn into_vec(self) -> Vec<String> {
        self.args
    }
}

pub struct CommandSpec {
    args: CommandArgs,
    content: Option<String>,
}

impl CommandSpec {
    pub fn new() -> Self {
        Self {
            args: CommandArgs::new(),
            content: None,
        }
    }

    pub fn with_help(mut self) -> Self {
        self.args.add_flag("help");
        self
    }

    pub fn with_verbose(mut self) -> Self {
        self.args.add_flag("verbose");
        self
    }

    pub fn with_project_filter(mut self, project_name: &str) -> Self {
        self.args.add_option("project", project_name);
        self
    }

    pub fn with_from_date_filter(mut self, from_date: &str) -> Self {
        self.args.add_option("from", from_date);
        self
    }

    pub fn with_content(self, content: &str) -> Self {
        Self {
            args: self.args,
            content: Some(content.to_string()),
        }
    }

    pub fn when_run(self) -> CommandResult {
        let (temp_dir, mut command) = match self.content {
            Some(content) => {
                let temp = Arc::new(
                    assert_fs::TempDir::new().expect("Failed to create temporary directory"),
                );
                let input_file = temp.child("test.md");
                input_file
                    .write_str(&content)
                    .expect("Failed to write to test file");

                let mut cmd = Command::cargo_bin("tt").expect("Failed to create cargo command");
                cmd.arg("--input").arg(input_file.path());

                (Some(temp), cmd)
            }
            None => {
                let cmd = Command::cargo_bin("tt").expect("Failed to create cargo command");
                (None, cmd)
            }
        };

        // Add all accumulated arguments
        command.args(self.args.into_vec());

        let output = command.assert();

        CommandResult {
            output,
            _temp_dir: temp_dir,
        }
    }
}

#[derive(Debug)]
struct Warning {
    file: String,
    line: Option<usize>,
    message: String,
}

impl Warning {
    fn new(message: &str) -> Self {
        Self {
            file: "test.md".to_string(), // default file
            line: None,
            message: message.to_string(),
        }
    }

    fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    fn with_file(mut self, file: &str) -> Self {
        self.file = file.to_string();
        self
    }

    fn to_pattern(&self) -> String {
        let line_part = match self.line {
            Some(line) => format!("line {}", line),
            None => "line \\d+".to_string(),
        };

        format!("Warning: {}: {}: {}", self.file, line_part, self.message)
    }
}

pub struct CommandResult {
    output: assert_cmd::assert::Assert,
    _temp_dir: Option<Arc<assert_fs::TempDir>>,
}

impl CommandResult {
    pub fn should_succeed(self) -> Self {
        Self {
            output: self.output.success(),
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_output(self, expected_output: &str) -> Self {
        let new_output = self
            .output
            .stdout(predicate::str::contains(expected_output));
        Self {
            output: new_output,
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_task(self, task_description: &str) -> Self {
        let escaped_description = regex::escape(task_description);
        let pattern = format!(r"\.*-\s+{}\.*", escaped_description);

        let new_output = self
            .output
            .stdout(predicate::str::is_match(pattern).unwrap());

        Self {
            output: new_output,
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_task_with_duration(
        self,
        task_description: &str,
        expected_duration: &str,
    ) -> Self {
        let escaped_description = regex::escape(task_description);
        let escaped_duration = regex::escape(expected_duration);
        let pattern = format!(
            r"-\s+{}\s*\.+\s*{}\s+\(\d+%\)",
            escaped_description, escaped_duration
        );

        let new_output = self
            .output
            .stdout(predicate::str::is_match(pattern).unwrap());

        Self {
            output: new_output,
            _temp_dir: self._temp_dir,
        }
    }

    fn expect_warning_pattern(self, pattern: &str) -> Self {
        Self {
            output: self
                .output
                .stdout(predicate::str::is_match(pattern).unwrap()),
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_warning(self, message: &str) -> Self {
        let warning = Warning::new(message);
        self.expect_warning_pattern(&warning.to_pattern())
    }

    pub fn expect_warning_at_line(self, line: usize, message: &str) -> Self {
        let warning = Warning::new(message).with_line(line);
        self.expect_warning_pattern(&warning.to_pattern())
    }

    pub fn expect_warning_with_file(self, file: &str, message: &str) -> Self {
        let warning = Warning::new(message).with_file(file);
        self.expect_warning_pattern(&warning.to_pattern())
    }

    pub fn expect_start_date(self, expected_start_date: &str) -> Self {
        let expected_output = format!("{} ->", expected_start_date);
        let new_output = self
            .output
            .stdout(predicate::str::contains(expected_output));
        Self {
            output: new_output,
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_end_date(self, expected_date: &str) -> Self {
        let expected_output = format!("-> {}", expected_date);
        let new_output = self
            .output
            .stdout(predicate::str::contains(expected_output));
        Self {
            output: new_output,
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_project(self, name: &str) -> ProjectAssertion {
        ProjectAssertion {
            cmd_result: self,
            project_name: name.to_string(),
            expectations: Vec::new(),
        }
    }

    pub fn expect_processing_output(self) -> Self {
        let new_output = self
            .output
            .stdout(predicate::str::contains("Processing path"));

        Self {
            output: new_output,
            ..self
        }
    }

    pub fn expect_no_data_found(self) -> Self {
        let new_output = self
            .output
            .stdout(predicate::str::contains("No data found."));

        Self {
            output: new_output,
            ..self
        }
    }

    fn assert_project(self, project_name: &str, expectations: &[(&str, String)]) -> Self {
        let project_name_with_delimiter = format!("{}.", project_name);

        let assert = self
            .output
            .stdout(predicate::function(move |output: &[u8]| {
                if let Ok(output_str) = std::str::from_utf8(output) {
                    let project_line = output_str
                        .lines()
                        .find(|line| line.contains(&project_name_with_delimiter));

                    match project_line {
                        Some(line) => {
                            let failed_expectations: Vec<_> = expectations
                                .iter()
                                .filter(|(_, expected)| !line.contains(expected))
                                .collect();

                            if !failed_expectations.is_empty() {
                                println!("\nProject '{}' validation failed", project_name);
                                println!("Found line: '{}'", line);
                                println!("Failed expectations:");
                                for (label, expected) in &failed_expectations {
                                    println!("  - {}: Expected '{}'", label, expected);
                                }
                                false
                            } else {
                                true
                            }
                        }
                        None => {
                            println!("\nProject '{}' not found in output", project_name);
                            false
                        }
                    }
                } else {
                    println!("\nInvalid UTF-8 in command output");
                    println!("Raw output: {:?}", output);
                    false
                }
            }));

        Self {
            output: assert,
            _temp_dir: self._temp_dir,
        }
    }
}

pub struct ProjectAssertion {
    cmd_result: CommandResult,
    project_name: String,
    expectations: Vec<(&'static str, String)>,
}

impl ProjectAssertion {
    pub fn taking(self, duration: &str) -> Self {
        let mut new_expectations = self.expectations;
        new_expectations.push(("Duration", duration.to_string()));
        Self {
            cmd_result: self.cmd_result,
            expectations: new_expectations,
            project_name: self.project_name,
        }
    }

    pub fn with_percentage(self, percentage: &str) -> Self {
        let formatted_percentage = format!("({:>3}%)", percentage);
        let mut new_expectations = self.expectations;
        new_expectations.push(("Percentage", formatted_percentage));
        Self {
            cmd_result: self.cmd_result,
            expectations: new_expectations,
            project_name: self.project_name,
        }
    }

    pub fn expect_project(self, name: &str) -> Self {
        // First validate the current project
        let cmd_result = self
            .cmd_result
            .assert_project(&self.project_name, &self.expectations);

        // Then create new ProjectAssertion for the next project
        Self {
            cmd_result,
            project_name: name.to_string(),
            expectations: Vec::new(),
        }
    }

    pub fn validate(self) -> CommandResult {
        let result = self
            .cmd_result
            .assert_project(&self.project_name, &self.expectations);
        result
    }
}
