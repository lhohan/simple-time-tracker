use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fmt;

pub struct CommandBuilder {
    args: Vec<String>,
    content: Option<String>,
}

pub struct CommandResult {
    output: assert_cmd::assert::Assert,
    _temp_dir: Option<assert_fs::TempDir>,
}

impl CommandBuilder {
    pub fn with_help() -> Self {
        Self {
            args: vec!["--help".to_string()],
            content: None,
        }
    }

    pub fn with_content(content: &str) -> Self {
        Self {
            args: Vec::new(),
            content: Some(content.to_string()),
        }
    }

    pub fn run(self) -> Result<CommandResult, Box<dyn std::error::Error>> {
        let (temp_dir, mut command) = match self.content {
            Some(content) => {
                let temp = assert_fs::TempDir::new()?;
                let input_file = temp.child("test.md");
                input_file.write_str(&content)?;

                let mut cmd = Command::cargo_bin("tt")?;
                cmd.arg("--input").arg(input_file.path());
                cmd.args(&self.args);

                (Some(temp), cmd)
            }
            None => {
                let mut cmd = Command::cargo_bin("tt")?;
                cmd.args(&self.args);
                (None, cmd)
            }
        };

        let output = command.assert();

        Ok(CommandResult {
            output,
            _temp_dir: temp_dir,
        })
    }
}

impl CommandResult {
    pub fn should_succeed(self) -> Self {
        Self {
            output: self.output.success(),
            _temp_dir: self._temp_dir,
        }
    }

    pub fn should_contain_project<const N: usize>(
        self,
        project: &str,
        expectations: [(&'static str, &str); N],
    ) -> Self {
        self.assert_project(&ProjectExpectations {
            name: project,
            expectations: expectations.into_iter().collect(),
        })
    }

    pub fn should_have_project(self, project: &str) -> Self {
        self.assert_project(&ProjectExpectations::new(project))
    }

    pub fn with_project(self, name: &str) -> ProjectAssertion<'_> {
        ProjectAssertion {
            cmd_result: self,
            project: ProjectExpectations::new(name),
        }
    }

    fn assert_project(self, project: &ProjectExpectations) -> Self {
        let name = project.name.to_string();
        let expectations = project.expectations.clone();

        let assert = self
            .output
            .stdout(predicate::function(move |output: &[u8]| {
                if let Ok(output_str) = std::str::from_utf8(output) {
                    output_str
                        .lines()
                        .find(|line| line.contains(&name))
                        .map_or(false, |line| {
                            let mut all_match = true;
                            let mut error = AssertionError::new(format!(
                                "Project '{}' validation failed",
                                name
                            ))
                            .actual(line);

                            for &(label, expected) in &expectations {
                                let found = line.contains(expected);
                                if !found {
                                    all_match = false;
                                    error = error.expected(label, expected.to_string());
                                }
                            }

                            if !all_match {
                                println!("{}", error);
                            }
                            all_match
                        })
                } else {
                    println!("\nInvalid UTF-8 in command output");
                    false
                }
            }));

        Self {
            output: assert,
            _temp_dir: self._temp_dir,
        }
    }
}

pub struct ProjectExpectations<'a> {
    name: &'a str,
    expectations: Vec<(&'static str, &'a str)>,
}

impl<'a> ProjectExpectations<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            expectations: Vec::new(),
        }
    }

    fn with_duration(mut self, duration: &'a str) -> Self {
        self.expectations.push(("Duration", duration));
        self
    }

    fn with_percentage(mut self, percentage: &'a str) -> Self {
        self.expectations.push(("Percentage", percentage));
        self
    }
}

pub struct ProjectAssertion<'a> {
    cmd_result: CommandResult,
    project: ProjectExpectations<'a>,
}

impl<'a> ProjectAssertion<'a> {
    pub fn taking(mut self, duration: &'a str) -> Self {
        self.project = self.project.with_duration(duration);
        self
    }

    pub fn with_percentage(mut self, percentage: &'a str) -> Self {
        self.project = self.project.with_percentage(percentage);
        self
    }

    pub fn and(self) -> CommandResult {
        self.cmd_result.assert_project(&self.project)
    }
}

pub struct Cmd;

impl Cmd {
    pub fn with_help() -> CommandBuilder {
        CommandBuilder::with_help()
    }

    pub fn given_content(content: &str) -> CommandBuilder {
        CommandBuilder::with_content(content)
    }
}

#[derive(Debug)]
struct AssertionError {
    context: String,
    expected: Vec<(&'static str, String)>,
    actual: String,
    full_output: String,
}

impl AssertionError {
    fn new(context: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            expected: Vec::new(),
            actual: String::new(),
            full_output: String::new(),
        }
    }

    fn expected(mut self, label: &'static str, value: impl Into<String>) -> Self {
        self.expected.push((label, value.into()));
        self
    }

    fn actual(mut self, line: impl Into<String>) -> Self {
        self.actual = line.into();
        self
    }

    fn full_output(mut self, output: impl Into<String>) -> Self {
        self.full_output = output.into();
        self
    }
}

impl fmt::Display for AssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", self.context)?;

        if !self.actual.is_empty() {
            writeln!(f, "Found line: '{}'", self.actual)?;
        }

        if !self.expected.is_empty() {
            writeln!(f, "Expected to contain:")?;
            for (label, value) in &self.expected {
                writeln!(f, "  - {}: '{}'", label, value)?;
            }
        }

        if !self.full_output.is_empty() {
            writeln!(f, "\nFull output:")?;
            writeln!(f, "{}", self.full_output)?;
        }

        Ok(())
    }
}
