use anyhow::{self, Context};

pub struct LinesCounter {
    pub lines: i32,
}

impl LinesCounter {
    pub fn new() -> LinesCounter {
        LinesCounter { lines: 0 }
    }

    pub fn count_from_pathbuf(&mut self, pathbuf: &std::path::PathBuf) -> anyhow::Result<()> {
        let entry = std::fs::read_to_string(&pathbuf).context(format!(
            "Failed to open file for read. {}",
            &pathbuf.display()
        ))?;

        entry.lines().for_each(|_| self.lines += 1);

        Ok(())
    }
}
