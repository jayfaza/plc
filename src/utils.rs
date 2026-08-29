#[allow(unused_imports)]
use crate::errors::HomeDirectoryError;
#[allow(unused_imports)]
use error_stack::{Report, ResultExt};
use std::fs;

pub fn calonicalize(path: &str) -> Result<String, Report<HomeDirectoryError>> {
    let pathbuf = fs::canonicalize(path)
        .attach_with(|| format!("failed to cononicalize path: {}", path))
        .change_context(HomeDirectoryError)?;
    Ok(pathbuf.display().to_string())
}
