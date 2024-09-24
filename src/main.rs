#[macro_use]
extern crate log;

use app::App;
use clap::Parser;

// Crate modules
mod api;
mod app;
mod cache;
mod cli;
mod config;
mod data;
mod display;
mod logging;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::run(cli::Cli::parse()).await
}
