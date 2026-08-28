use anyhow::Context;

use crate::app::App;

mod app;
mod config;
mod directory_manager;
mod lines_couter;
mod parser;

fn main() -> anyhow::Result<()> {
    App::run().context("Failed to start app.")
}
