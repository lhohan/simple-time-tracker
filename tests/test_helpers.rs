#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
use assert_cmd::Command;
use assert_fs::prelude::*;
use chrono::NaiveDate;
use predicates::prelude::*;
use std::{path::PathBuf, sync::Arc};

#[derive(Default, Clone)]
struct CommandArgs {
    args: Vec<String>,
}

impl CommandArgs {
    fn new() -> Self {
        Self::default()
    }

    fn add_flag(&mut self, flag: &str) {
        self.args.push(format!("--{flag}"));
    }

    fn add_option(&mut self, option: &str, value: &str) {
        self.args.push(format!("--{option}"));
        self.args.push(value.to_string());
    }

    fn into_vec(self) -> Vec<String> {
        self.args
    }
}

#[derive(Debug, Clone)]
enum InputSource {
    File {
        content: String,
        path: std::path::PathBuf,
    },
    Directory {
        files: Vec<InputSource>,
    },
}

impl InputSource {
    fn file(content: &str) -> Self {
        Self::named_file("test.md", content)
    }

    fn path_file(path: impl AsRef<std::path::Path>, content: &str) -> Self {
        Self::File {
            content: content.to_string(),
            path: path.as_ref().to_path_buf(),
        }
    }

    fn named_file(name: &str, content: &str) -> Self {
        Self::File {
            content: content.to_string(),
            path: name.into(),
        }
    }

    fn directory(files: Vec<InputSource>) -> Self {
        Self::Directory { files }
    }
}

// Group command and the test files location together to couple their life time.
struct ExecutionContext {
    command: Command,
    _temp_dir: Option<Arc<assert_fs::TempDir>>,
}

impl ExecutionContext {
    fn execute(mut self) -> CommandResult {
        let output = self.command.assert();
        std::env::remove_var("TT_TODAY"); // cleanup environment
        CommandResult { output }
    }

    fn run_on_date(&mut self, date: NaiveDate) -> &Self {
        let today = date.format("%Y-%m-%d").to_string();
        self.command.env("TT_TODAY", today);
        self
    }
}

#[derive(Clone)]
pub struct CommandSpec {
    args: CommandArgs,
    input: Option<InputSource>,
    run_date: Option<NaiveDate>,
}

impl Default for CommandSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandSpec {
    pub fn new() -> Self {
        Self {
            args: CommandArgs::new(),
            input: None,
            run_date: None,
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

    pub fn with_details(mut self) -> Self {
        self.args.add_flag("details");
        self
    }

    pub fn with_limit(mut self) -> Self {
        self.args.add_flag("limit");
        self
    }

    pub fn with_period(mut self, period: &str) -> Self {
        self.args.add_option("period", period);
        self
    }

    pub fn with_filter_project(mut self, project_name: &str) -> Self {
        self.args.add_option("project", project_name);
        self
    }

    pub fn with_filter_from_date(mut self, from_date: &str) -> Self {
        self.args.add_option("from", from_date);
        self
    }

    pub fn with_tags_filter(mut self, tags: &[&str]) -> Self {
        self.args.add_option("tags", &tags.join(","));
        self
    }

    pub fn with_exclude_tags_filter(mut self, tags: &[&str]) -> Self {
        self.args.add_option("exclude-tags", &tags.join(","));
        self
    }

    pub fn with_format(mut self, format: &str) -> Self {
        self.args.add_option("format", format);
        self
    }

    pub fn with_directory_containing_files(mut self, files: &[(&str, &str)]) -> Self {
        let files = files
            .iter()
            .map(|(name, content)| InputSource::path_file(name, content))
            .collect();

        self.input = Some(InputSource::directory(files));
        self
    }

    pub fn with_file(mut self, content: &str) -> Self {
        self.input = Some(InputSource::file(content));
        self
    }

    pub fn at_date(mut self, date: &str) -> Self {
        let date =
            NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Invalid date format in test");
        self.run_date = Some(date);
        self
    }

    fn create_base_command() -> Command {
        Command::cargo_bin("tt").expect("Failed to create cargo command")
    }

    fn setup_test_files(input: InputSource) -> (Arc<assert_fs::TempDir>, PathBuf) {
        let temp =
            Arc::new(assert_fs::TempDir::new().expect("Failed to create temporary directory"));

        match input {
            InputSource::File {
                content,
                path: name,
            } => {
                let input_file = temp.child(&name);
                input_file
                    .write_str(&content)
                    .expect("Failed to write to test file");
                (temp, input_file.path().to_path_buf())
            }
            InputSource::Directory { files } => {
                // Create all files in the directory
                for file in files {
                    match file {
                        InputSource::File { content, path } => {
                            // create parent directories if not exist
                            if let Some(parent) = path.parent() {
                                temp.child(parent)
                                    .create_dir_all()
                                    .expect("Failed to create parent directories");
                            }
                            let file_path = temp.child(&path);
                            file_path
                                .write_str(&content)
                                .expect("Failed to write to test file");
                        }
                        InputSource::Directory { .. } => {
                            panic!("Nested directories not supported yet");
                        }
                    }
                }
                (temp.clone(), temp.path().to_path_buf())
            }
        }
    }

    pub fn when_run(self) -> CommandResult {
        let mut command = Self::create_base_command();
        let temp_dir = self.input.map(|input| {
            let (temp_dir, input_path) = Self::setup_test_files(input);
            command.arg("--input").arg(input_path);
            temp_dir
        });
        command.args(self.args.clone().into_vec());

        let mut context = ExecutionContext {
            command,
            _temp_dir: temp_dir,
        };
        if let Some(run_date) = self.run_date {
            context.run_on_date(run_date);
        }

        context.execute()
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
            Some(line) => format!("line {line}"),
            None => "line \\d+".to_string(),
        };

        format!("Warning: {}: {}: {}", self.file, line_part, self.message)
    }
}

pub struct CommandResult {
    pub output: assert_cmd::assert::Assert,
}

impl CommandResult {
    pub fn should_succeed(self) -> Self {
        Self {
            output: self.output.success(),
        }
    }

    pub fn should_fail(self) -> Self {
        Self {
            output: self.output.failure(),
        }
    }

    pub fn expect_error(self, expected_output: &str) -> Self {
        let new_output = self
            .output
            .stderr(predicate::str::contains(expected_output));
        Self { output: new_output }
    }

    pub fn expect_output(self, expected_output: &str) -> Self {
        let new_output = self
            .output
            .stdout(predicate::str::contains(expected_output));
        Self { output: new_output }
    }

    pub fn expect_task(self, task_description: &str) -> Self {
        let escaped_description = regex::escape(task_description);
        let pattern = format!(r"\.*-\s+{escaped_description}\.*");

        let new_output = self
            .output
            .stdout(predicate::str::is_match(pattern).unwrap());

        Self { output: new_output }
    }

    pub fn expect_task_with_duration(
        self,
        task_description: &str,
        expected_duration: &str,
    ) -> Self {
        let escaped_description = regex::escape(task_description);
        let escaped_duration = regex::escape(expected_duration);
        let pattern = format!(r"-\s+{escaped_description}\s*\.+\s*{escaped_duration}\s+\(\d+%\)",);

        let new_output = self
            .output
            .stdout(predicate::str::is_match(pattern).unwrap());

        Self { output: new_output }
    }

    fn expect_warning_pattern(self, pattern: &str) -> Self {
        Self {
            output: self
                .output
                .stdout(predicate::str::is_match(pattern).unwrap()),
        }
    }

    pub fn expect_no_warnings(self) -> Self {
        Self {
            output: self
                .output
                .stdout(predicate::str::contains("Warning:").not()),
        }
    }

    pub fn expect_no_text(self, text: &str) -> Self {
        Self {
            output: self.output.stdout(predicate::str::contains(text).not()),
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
        let expected_output = format!("{expected_start_date} ->");
        let new_output = self
            .output
            .stdout(predicate::str::contains(expected_output));
        Self { output: new_output }
    }

    pub fn expect_end_date(self, expected_date: &str) -> Self {
        let expected_output = format!("-> {expected_date}");
        let new_output = self
            .output
            .stdout(predicate::str::contains(expected_output));
        Self { output: new_output }
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
        let project_name_with_delimiter = format!("{project_name}.");

        let assert = self
            .output
            .stdout(predicate::function(move |output: &[u8]| {
                if let Ok(output_str) = std::str::from_utf8(output) {
                    let project_line = output_str
                        .lines()
                        .find(|line| line.contains(&project_name_with_delimiter));

                    if let Some(line) = project_line {
                        let failed_expectations: Vec<_> = expectations
                            .iter()
                            .filter(|(_, expected)| !line.contains(expected))
                            .collect();

                        if failed_expectations.is_empty() {
                            true
                        } else {
                            println!("\nProject '{project_name}' validation failed");
                            println!("Found line: '{line}'");
                            println!("Failed expectations:");
                            for (label, expected) in &failed_expectations {
                                println!("  - {label}: Expected '{expected}'");
                            }
                            false
                        }
                    } else {
                        println!("\nProject '{project_name}' not found in output");
                        false
                    }
                } else {
                    println!("\nInvalid UTF-8 in command output");
                    println!("Raw output: {output:?}");
                    false
                }
            }));

        Self { output: assert }
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
        let formatted_percentage = format!("({percentage:>3}%)");
        let mut new_expectations = self.expectations;
        new_expectations.push(("Percentage", formatted_percentage));
        Self {
            cmd_result: self.cmd_result,
            expectations: new_expectations,
            project_name: self.project_name,
        }
    }

    pub fn with_context(self, expected_tag: &str) -> Self {
        let mut new_expectations = self.expectations;
        new_expectations.push(("Context", expected_tag.to_string()));
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
        self.cmd_result
            .assert_project(&self.project_name, &self.expectations)
    }
}
