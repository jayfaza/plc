use crate::config::ALLOWEDFILEEXTENSIONS;
use crate::errors::WalkDirError;
use error_stack::Report;
use walkdir::WalkDir;

pub struct DirectoryManager {}

impl DirectoryManager {
    pub fn new() -> DirectoryManager {
        return DirectoryManager {};
    }

    pub fn collect_paths(
        &self,
        directory: &str,
    ) -> Result<Vec<std::path::PathBuf>, Report<WalkDirError>> {
        let collected_paths: Vec<std::path::PathBuf> = WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path().extension().is_some_and(|p| {
                    ALLOWEDFILEEXTENSIONS.contains(&p.display().to_string().as_str())
                })
            })
            .map(|e| e.into_path())
            .collect();
        Ok(collected_paths)
    }
}
