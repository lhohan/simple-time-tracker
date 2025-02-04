use crate::domain::ParseError;
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

pub(super) trait FileProcessor {
    fn process<F>(&self, path: &Path, process_content: F) -> Result<(), ParseError>
    where
        F: FnMut(&str, &str) -> Result<(), ParseError>;
}

#[derive(Debug)]
pub(super) struct SingleFileProcessor;

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

#[derive(Debug)]
pub enum ProcessType {
    File(SingleFileProcessor),
    Directory(DirectoryProcessor),
}

impl FileProcessor for ProcessType {
    fn process<F>(&self, path: &Path, mut process_content: F) -> Result<(), ParseError>
    where
        F: FnMut(&str, &str) -> Result<(), ParseError>,
    {
        match &self {
            ProcessType::File(single_file_processor) => {
                single_file_processor.process(path, process_content)
            }
            ProcessType::Directory(directory_processor) => {
                dbg!("PROCESSONG DIR");
                for entry in WalkDir::new(path)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        dbg!(e);
                        e.path()
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| ext == "md" || ext == "txt")
                            .unwrap_or(false)
                    })
                {
                    directory_processor
                        .file_processor
                        .process(entry.path(), &mut process_content)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct DirectoryProcessor {
    file_processor: SingleFileProcessor,
}

impl DirectoryProcessor {
    pub fn new() -> Self {
        Self {
            file_processor: SingleFileProcessor,
        }
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
