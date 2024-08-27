use anyhow::Result;
use clap::Parser;

use crate::{cli::Cli, config::Config};

pub struct App {}

impl App {
    pub fn run() -> Result<()> {
        // Load environment variables from .env file
        dotenv::dotenv()?;
        let options = Cli::parse();
        let config = Config::from_cli(options)?;
        println!("{config:?}");
        Ok(())
    }
}
