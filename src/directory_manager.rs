use crate::config::Config;
use crate::errors::WalkDirError;
use error_stack::{Report, ResultExt};
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
        let included_exts = Config::new()
            .load_config()
            .change_context(WalkDirError)?
            .included_extensions;
        let collected_paths: Vec<std::path::PathBuf> = WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .is_some_and(|p| included_exts.clone().contains(&p.display().to_string()))
            })
            .map(|e| e.into_path())
            .collect();
        Ok(collected_paths)
    }
}
