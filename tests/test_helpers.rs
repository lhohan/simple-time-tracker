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
    _temp_dir: Option<assert_fs::TempDir>, // Keep temp dir alive during test
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
            // Made command mutable
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
        let expectations_vec: Vec<_> = expectations.into_iter().collect();
        let assert = self
            .output
            .stdout(predicate::function(move |output: &[u8]| {
                if let Ok(output_str) = std::str::from_utf8(output) {
                    output_str
                        .lines()
                        .find(|line| line.contains(project))
                        .map_or(false, |line| {
                            let mut all_match = true;
                            let mut error = AssertionError::new(format!(
                                "Project '{}' validation failed",
                                project
                            ))
                            .actual(line);

                            for &(label, expected) in &expectations_vec {
                                let found = line.contains(expected);
                                if !found {
                                    all_match = false;
                                    error = error.expected(label, expected);
                                }
                            }

                            if !all_match {
                                println!("{}", error);
                            }
                            all_match
                        })
                } else {
                    false
                }
            }));

        Self {
            output: assert,
            _temp_dir: self._temp_dir,
        }
    }
}

pub struct Cmd;

impl Cmd {
    pub fn with_help() -> CommandBuilder {
        CommandBuilder::with_help()
    }

    pub fn with_content(content: &str) -> CommandBuilder {
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
