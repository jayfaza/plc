use std::fs;

use crate::errors::LinesCountError;
use error_stack::{Report, ResultExt};

pub struct LinesCounter {
    pub lines: i32,
}

impl LinesCounter {
    pub fn new() -> LinesCounter {
        LinesCounter { lines: 0 }
    }

    pub fn count_from_pathbuf(
        &mut self,
        pathbuf: &std::path::PathBuf,
    ) -> Result<(), Report<LinesCountError>> {
        let entry = fs::read_to_string(pathbuf)
            .attach_with(|| format!("failed to open file at path: {:?}", pathbuf))
            .change_context(LinesCountError)?;

        entry.lines().for_each(|_| self.lines += 1);

        Ok(())
    }
}
