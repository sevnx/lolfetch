#![allow(unused)]

use anyhow::Result;
use app::App;

// Crate modules
mod api;
mod app;
mod cache;
mod cli;
mod config;
mod data;
mod display;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    App::run().await
}
