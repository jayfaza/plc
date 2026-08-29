use crate::app::App;
use crate::errors::ExecutionError;
use error_stack::Report;

mod app;
mod config;
mod directory_manager;
mod errors;
mod lines_couter;
mod parser;
mod utils;

fn main() -> Result<(), Report<ExecutionError>> {
    App::run()
}
