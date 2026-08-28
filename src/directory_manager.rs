use crate::config::ALLOWEDFILEEXTENSIONS;
use walkdir::WalkDir;

pub struct DirectoryManager {}

impl DirectoryManager {
    pub fn new() -> DirectoryManager {
        return DirectoryManager {};
    }

    pub fn collect_paths(&self, directory: &str) -> anyhow::Result<Vec<std::path::PathBuf>> {
        let collected_paths: Vec<std::path::PathBuf> = WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .is_some_and(|e| ALLOWEDFILEEXTENSIONS.contains(&e.to_str().unwrap()))
            })
            .map(|e| e.into_path())
            .collect();
        Ok(collected_paths)
    }
}
