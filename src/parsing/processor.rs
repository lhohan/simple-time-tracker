use crate::domain::ParseError;
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

pub(super) trait FileProcessor {
    fn process<F>(&self, path: &Path, processor: F) -> Result<(), ParseError>
    where
        F: FnMut(ProcessingInput) -> Result<(), ParseError>;
}

mod processors {
    use super::*;

    #[derive(Debug)]
    pub struct ProcessingInput {
        content: String,
        file_name: String,
    }

    impl ProcessingInput {
        pub fn new(content: String, file_name: String) -> Self {
            Self { content, file_name }
        }

        pub fn content(&self) -> &str {
            &self.content
        }

        pub fn file_name(&self) -> &str {
            &self.file_name
        }
    }

    #[derive(Debug)]
    pub(crate) struct SingleFileProcessor;

    impl SingleFileProcessor {
        fn read_file_content(path: &Path) -> Result<String, ParseError> {
            read_to_string(path).map_err(|err| {
                ParseError::ErrorReading(format!("Failed to read {}: {}", path.display(), err))
            })
        }

        fn extract_file_name(path: &Path) -> Result<String, ParseError> {
            path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| {
                    ParseError::ErrorReading(format!("Invalid filename: {}", path.display()))
                })
                .map(String::from)
        }
    }

    impl FileProcessor for SingleFileProcessor {
        fn process<F>(&self, path: &Path, mut processor: F) -> Result<(), ParseError>
        where
            F: FnMut(ProcessingInput) -> Result<(), ParseError>,
        {
            let file_name = Self::extract_file_name(path)?;
            let content = Self::read_file_content(path)?;

            processor(ProcessingInput::new(content, file_name))
        }
    }

    #[derive(Debug)]
    pub(crate) struct DirectoryProcessor {
        file_processor: SingleFileProcessor,
    }

    impl DirectoryProcessor {
        pub(super) fn new() -> Self {
            Self {
                file_processor: SingleFileProcessor,
            }
        }
    }

    impl FileProcessor for DirectoryProcessor {
        fn process<F>(&self, path: &Path, mut processor: F) -> Result<(), ParseError>
        where
            F: FnMut(ProcessingInput) -> Result<(), ParseError>,
        {
            for entry in WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| is_supported_file(e.path()))
            {
                self.file_processor.process(entry.path(), &mut processor)?;
            }
            Ok(())
        }
    }

    impl FileProcessor for InputProcessor {
        fn process<F>(&self, path: &Path, processor: F) -> Result<(), ParseError>
        where
            F: FnMut(ProcessingInput) -> Result<(), ParseError>,
        {
            match self {
                InputProcessor::File(file_processor) => file_processor.process(path, processor),
                InputProcessor::Directory(dir_processor) => dir_processor.process(path, processor),
            }
        }
    }

    const SUPPORTED_EXTENSIONS: [&str; 2] = ["md", "txt"];

    fn is_supported_file(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
            .unwrap_or(false)
    }
}

use processors::*;

#[derive(Debug)]
pub enum InputProcessor {
    File(SingleFileProcessor),
    Directory(DirectoryProcessor),
}

impl InputProcessor {
    pub fn from_path(path: &Path) -> Self {
        if path.is_dir() {
            InputProcessor::Directory(DirectoryProcessor::new())
        } else {
            InputProcessor::File(SingleFileProcessor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn processes_single_markdown_file() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.with_file("test.md", "test content")?;

        test.process(test.temp_dir.child("test.md").path())
            .expect_success()
            .expect_processed_exactly(1)
            .expect_processed_file("test.md", "test content");

        Ok(())
    }

    #[test]
    fn processes_only_supported_files_in_directory() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.with_file("test.md", "md content")?
            .with_file("test.txt", "txt content")?
            .with_file("test.other", "other content")?;

        test.process(test.temp_dir.path())
            .expect_success()
            .expect_processed_exactly(2)
            .expect_only_processed_extensions(&["md", "txt"]);

        Ok(())
    }

    #[test]
    fn handles_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.process(test.temp_dir.path())
            .expect_success()
            .expect_processed_exactly(0);

        Ok(())
    }

    #[test]
    fn processes_nested_directories() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.with_directory("subdir")?
            .with_file("test.md", "root content")?
            .with_file("subdir/test.md", "nested content")?;

        test.process(test.temp_dir.path())
            .expect_success()
            .expect_processed_exactly(2)
            .expect_processed_file("test.md", "root content")
            .expect_processed_file("test.md", "nested content");

        Ok(())
    }

    #[test]
    fn handles_non_existent_file() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;
        let non_existent = test.temp_dir.path().join("does_not_exist.md");

        test.process(&non_existent)
            .expect_error_containing("Failed to read");

        Ok(())
    }

    struct ProcessingTest {
        temp_dir: assert_fs::TempDir,
    }

    enum ProcessingOutcome {
        Success(ProcessingResults),
        Error(ParseError),
    }

    impl ProcessingTest {
        // Modified to return our new ProcessingOutcome
        fn process(&self, path: &Path) -> ProcessingOutcome {
            let processor = InputProcessor::from_path(path);
            let mut files = Vec::new();

            match processor.process(path, |input| {
                files.push((input.file_name().to_string(), input.content().to_string()));
                Ok(())
            }) {
                Ok(()) => ProcessingOutcome::Success(ProcessingResults { files }),
                Err(e) => ProcessingOutcome::Error(e),
            }
        }
    }

    // Enhanced DSL for assertions
    trait ProcessingAssertions {
        fn expect_success(self) -> ProcessingResults;
        fn expect_error(self) -> ParseError;
        fn expect_error_containing(self, message: &str) -> ParseError;
    }

    impl ProcessingAssertions for ProcessingOutcome {
        fn expect_success(self) -> ProcessingResults {
            match self {
                ProcessingOutcome::Success(results) => results,
                ProcessingOutcome::Error(e) => panic!("Expected success but got error: {:?}", e),
            }
        }

        fn expect_error(self) -> ParseError {
            match self {
                ProcessingOutcome::Success(results) => {
                    dbg!(&results.files);
                    panic!(
                        "Expected error but got success with {} files",
                        results.files.len()
                    )
                }
                ProcessingOutcome::Error(e) => e,
            }
        }

        fn expect_error_containing(self, message: &str) -> ParseError {
            let error = self.expect_error();
            match &error {
                ParseError::ErrorReading(msg) if msg.contains(message) => error,
                _ => panic!("Error message '{}' did not contain '{}'", error, message),
            }
        }
    }

    struct ProcessingResults {
        files: Vec<(String, String)>,
    }

    impl ProcessingTest {
        fn new() -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self {
                temp_dir: assert_fs::TempDir::new()?,
            })
        }

        fn with_file(
            &self,
            name: &str,
            content: &str,
        ) -> Result<&Self, Box<dyn std::error::Error>> {
            self.temp_dir.child(name).write_str(content)?;
            Ok(self)
        }

        fn with_directory(&self, name: &str) -> Result<&Self, Box<dyn std::error::Error>> {
            self.temp_dir.child(name).create_dir_all()?;
            Ok(self)
        }
    }

    impl ProcessingResults {
        fn expect_processed_exactly(self, expected_count: usize) -> Self {
            assert_eq!(
                self.files.len(),
                expected_count,
                "Expected to process {} files, but processed {}",
                expected_count,
                self.files.len()
            );
            self
        }

        fn expect_processed_file(self, name: &str, content: &str) -> Self {
            assert!(
                self.files
                    .iter()
                    .any(|(n, c)| n.ends_with(name) && c == content),
                "Expected to find file '{name}' with content '{content}'",
            );
            self
        }

        fn expect_only_processed_extensions(self, extensions: &[&str]) -> Self {
            for (name, _) in &self.files {
                let ext = Path::new(name)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                assert!(
                    extensions.contains(&ext),
                    "Processed file with unexpected extension: {name}"
                );
            }
            self
        }
    }
}
