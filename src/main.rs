mod app;
mod config;
mod directory_manager;
mod errors;
mod lines_couter;
mod parser;
mod utils;

use error_stack::Report;
use crate::app::App;
use crate::errors::ExecutionError;

fn main() -> Result<(), Report<ExecutionError>> {
    App::run()
}
