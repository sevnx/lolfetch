#![allow(unused)]

use anyhow::Result;
use app::App;

#[macro_use]
extern crate log;

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
async fn main() -> Result<()> {
    App::run().await
}
