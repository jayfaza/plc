use std::process::Termination;

use crate::errors::{ExecutionError, PathDoesNotExists, PathIsNotADirectory};
use crate::utils::calonicalize;
use error_stack::{Report, ResultExt};

use clap::Parser;

use crate::directory_manager::DirectoryManager;
use crate::lines_couter::LinesCounter;
use crate::parser::Args;
use shellexpand::tilde;

pub struct App {}

impl App {
    pub fn run() -> Result<(), Report<ExecutionError>> {
        let mut directory = Args::parse().project;

        directory = tilde(&directory).to_string();

        if !std::path::PathBuf::from(&directory).exists() {
            return Err(Report::new(PathDoesNotExists))
                .attach_with(|| format!("path: {}", directory))
                .change_context(ExecutionError)?;
        }

        if !std::path::PathBuf::from(&directory).is_dir() {
            return Err(Report::new(PathIsNotADirectory)).change_context(ExecutionError)?;
        }

        directory = calonicalize(&directory).change_context(ExecutionError)?;

        let directory_manager = DirectoryManager::new();
        let directory_file_paths = directory_manager
            .collect_paths(&directory)
            .change_context(ExecutionError)?;

        let mut lines_counter = LinesCounter::new();

        directory_file_paths.iter().for_each(|p| {
            lines_counter
                .count_from_pathbuf(p)
                .change_context(ExecutionError)
                .report();
        });

        println!("{} lines in {}", lines_counter.lines, &directory);
        Ok(())
    }
}
