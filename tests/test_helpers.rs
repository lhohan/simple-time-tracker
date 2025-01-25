use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

pub struct CommandSpec {
    args: Vec<String>,
    content: Option<String>,
}

impl CommandSpec {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            content: None,
        }
    }

    pub fn with_help(self) -> Self {
        let mut args = self.args;
        args.push("--help".to_string());
        Self {
            args,
            content: self.content,
        }
    }

    pub fn with_verbose(self) -> Self {
        let mut args = self.args;
        args.push("--verbose".to_string());
        Self {
            args,
            content: self.content,
        }
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
                let temp = assert_fs::TempDir::new().expect("Failed to create temporary directory");
                let input_file = temp.child("test.md");
                input_file
                    .write_str(&content)
                    .expect("Failed to write to test file");

                let mut cmd = Command::cargo_bin("tt").expect("Failed to create cargo command");
                cmd.arg("--input").arg(input_file.path());
                cmd.args(&self.args);

                (Some(temp), cmd)
            }
            None => {
                let mut cmd = Command::cargo_bin("tt").expect("Failed to create cargo command");
                cmd.args(&self.args);
                (None, cmd)
            }
        };

        let output = command.assert();

        CommandResult {
            output,
            _temp_dir: temp_dir,
        }
    }
}

pub struct CommandResult {
    output: assert_cmd::assert::Assert,
    _temp_dir: Option<assert_fs::TempDir>,
}

pub struct ProjectExpectations {
    name: String,
    expectations: Vec<(&'static str, String)>,
}

impl ProjectExpectations {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            expectations: Vec::new(),
        }
    }

    fn with_duration(mut self, duration: &str) -> Self {
        self.expectations.push(("Duration", duration.to_string()));
        self
    }

    fn with_percentage(mut self, percentage: String) -> Self {
        self.expectations.push(("Percentage", percentage));
        self
    }
}

pub struct ProjectAssertion {
    cmd_result: CommandResult,
    project_expectations: ProjectExpectations,
}

impl CommandResult {
    pub fn should_succeed(self) -> Self {
        Self {
            output: self.output.success(),
            _temp_dir: self._temp_dir,
        }
    }

    pub fn expect_project(self, name: &str) -> ProjectAssertion {
        ProjectAssertion {
            cmd_result: self,
            project_expectations: ProjectExpectations::new(name),
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

    pub fn should_contain_project<const N: usize>(
        self,
        project: &str,
        expectations: [(&'static str, &str); N],
    ) -> Self {
        let project_exp = ProjectExpectations {
            name: project.to_string(),
            expectations: expectations
                .into_iter()
                .map(|(label, value)| (label, value.to_string()))
                .collect(),
        };

        self.assert_project(&project_exp)
    }

    fn assert_project(self, project: &ProjectExpectations) -> Self {
        let project_name = &project.name;
        let project_name_with_delimiter = &format!("{}.", project_name);
        let expectations = &project.expectations;

        let assert = self
            .output
            .stdout(predicate::function(move |output: &[u8]| {
                if let Ok(output_str) = std::str::from_utf8(output) {
                    let project_line = output_str
                        .lines()
                        .find(|line| line.contains(project_name_with_delimiter));

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
                            println!(
                                "Expected to find a line containing: '{}'",
                                project_name_with_delimiter
                            );
                            println!("Full output:");
                            println!("---");
                            println!("{}", output_str);
                            println!("---");
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

impl ProjectAssertion {
    pub fn taking(mut self, duration: &str) -> Self {
        self.project_expectations = self.project_expectations.with_duration(duration);
        self
    }

    pub fn with_percentage(mut self, percentage: &str) -> Self {
        let formatted_percentage = format!("({:>3}%)", percentage);
        self.project_expectations = self
            .project_expectations
            .with_percentage(formatted_percentage);
        self
    }

    pub fn expect_project(self, name: &str) -> Self {
        // First validate the current project
        let cmd_result = self.cmd_result.assert_project(&self.project_expectations);

        // Then create new ProjectAssertion for the next project
        Self {
            cmd_result,
            project_expectations: ProjectExpectations::new(name),
        }
    }

    pub fn validate(self) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd_result.assert_project(&self.project_expectations);
        Ok(())
    }
}
