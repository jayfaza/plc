use std::fs;

use clap::Parser;

use crate::directory_manager::DirectoryManager;
use crate::lines_couter::LinesCounter;
use crate::parser::Args;
use shellexpand::tilde;

pub struct App {}

impl App {
    pub fn run() -> anyhow::Result<()> {
        let mut directory = Args::parse().project;

        directory = tilde(&directory).to_string();
        directory = fs::canonicalize(&directory)?.to_str().unwrap().to_string();

        let directory_manager = DirectoryManager::new();
        let directory_file_paths = directory_manager.collect_paths(&directory).unwrap();
        let mut lines_counter = LinesCounter::new();

        directory_file_paths
            .iter()
            .for_each(|p| lines_counter.count_from_pathbuf(p).unwrap());

        println!("{} lines in {}", lines_counter.lines, &directory);
        Ok(())
    }
}
