use std::{error::Error, fmt};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ExecutionError;

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Execution eror")
    }
}

impl Error for ExecutionError {}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HomeDirectoryError;

impl fmt::Display for HomeDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to get your home directory")
    }
}

impl Error for HomeDirectoryError {}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WalkDirError;

impl fmt::Display for WalkDirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Walk dir execution was interrupted")
    }
}

impl Error for WalkDirError {}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LinesCountError;

impl fmt::Display for LinesCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to count file lines")
    }
}

impl Error for LinesCountError {}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PathDoesNotExists;

impl fmt::Display for PathDoesNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Path does not exists")
    }
}

impl Error for PathDoesNotExists {}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PathIsNotADirectory;

impl fmt::Display for PathIsNotADirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Path has to be a direcotry")
    }
}

impl Error for PathIsNotADirectory {}
