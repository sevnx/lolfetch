use anyhow::Result;
use clap::Parser;
use riven::{RiotApi, RiotApiConfig};

use crate::{api::Fetcher, cli::Cli, config::Config, data::ApplicationData, display::Layout};

pub struct App {}

impl App {
    pub async fn run() -> Result<()> {
        dotenv::dotenv()?;

        let options = Cli::parse();
        let config = Config::from_cli(options)?;
        let api = RiotApi::new(RiotApiConfig::with_key(&config.api_key));
        let data = api.fetch(&config).await?;
        let processed = ApplicationData::process(data, &config).await;

        Layout::new(processed).display()
    }
}
