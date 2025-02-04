use crate::domain::ParseError;
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

mod processors {
    use super::*;
    #[derive(Debug)]
    pub(crate) struct SingleFileProcessor;

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

    impl FileProcessor for SingleFileProcessor {
        fn process<F>(&self, path: &Path, mut process_content: F) -> Result<(), ParseError>
        where
            F: FnMut(&str, &str) -> Result<(), ParseError>,
        {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let content = read_to_string(path).map_err(|_| {
                ParseError::ErrorReading(
                    path.to_str()
                        .expect("Could not get path to file")
                        .to_string(),
                )
            })?;

            process_content(&content, &file_name)
        }
    }

    impl FileProcessor for DirectoryProcessor {
        fn process<F>(&self, path: &Path, mut process_content: F) -> Result<(), ParseError>
        where
            F: FnMut(&str, &str) -> Result<(), ParseError>,
        {
            for entry in WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "md" || ext == "txt")
                        .unwrap_or(false)
                })
            {
                self.file_processor
                    .process(entry.path(), &mut process_content)?;
            }
            Ok(())
        }
    }

    impl FileProcessor for InputProcessor {
        fn process<F>(&self, path: &Path, process_content: F) -> Result<(), ParseError>
        where
            F: FnMut(&str, &str) -> Result<(), ParseError>,
        {
            match self {
                InputProcessor::File(processor) => processor.process(path, process_content),
                InputProcessor::Directory(processor) => processor.process(path, process_content),
            }
        }
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

pub(super) trait FileProcessor {
    fn process<F>(&self, path: &Path, process_content: F) -> Result<(), ParseError>
    where
        F: FnMut(&str, &str) -> Result<(), ParseError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn processes_single_markdown_file() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.with_file("test.md", "test content")?;

        test.process(test.temp_dir.child("test.md").path())?
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

        test.process(test.temp_dir.path())?
            .expect_processed_exactly(2)
            .expect_only_processed_extensions(&["md", "txt"]);

        Ok(())
    }

    #[test]
    fn handles_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.process(test.temp_dir.path())?
            .expect_processed_exactly(0);

        Ok(())
    }

    #[test]
    fn processes_nested_directories() -> Result<(), Box<dyn std::error::Error>> {
        let test = ProcessingTest::new()?;

        test.with_directory("subdir")?
            .with_file("test.md", "root content")?
            .with_file("subdir/test.md", "nested content")?;

        test.process(test.temp_dir.path())?
            .expect_processed_exactly(2)
            .expect_processed_file("test.md", "root content")
            .expect_processed_file("test.md", "nested content");

        Ok(())
    }

    struct ProcessingTest {
        temp_dir: assert_fs::TempDir,
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

        fn process(&self, path: &Path) -> Result<ProcessingResults, ParseError> {
            let processor = InputProcessor::from_path(path);

            let mut files = Vec::new();
            processor.process(path, |content, name| {
                files.push((name.to_string(), content.to_string()));
                Ok(())
            })?;

            Ok(ProcessingResults { files })
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
                "Expected to find file '{}' with content '{}'",
                name,
                content
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
                    "Processed file with unexpected extension: {}",
                    name
                );
            }
            self
        }
    }
}
