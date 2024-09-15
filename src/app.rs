use anyhow::Result;
use clap::Parser;
use riven::{RiotApi, RiotApiConfig};

use crate::{
    api::Fetcher,
    cache,
    cli::{self, cache::CacheAction, Cli},
    config::{Config, Lolfetch},
    data::ApplicationData,
    display::Layout,
    logging,
};

pub struct App {}

impl App {
    pub async fn run() -> Result<()> {
        dotenv::dotenv()?;

        let options = Cli::parse();
        if options.verbose {
            logging::setup();
        }

        match options.command {
            None => {
                let config = Config::from_cli(cli::lolfetch::Lolfetch::parse())?;
                let api = RiotApi::new(RiotApiConfig::with_key(&config.api_key));
                let data = api.fetch(&config).await?;
                let processed = ApplicationData::process(data, &config).await?;
                info!("Displaying data");
                Layout::new(processed).display()?;
            }
            Some(command) => match command {
                cli::Commands::Cache(cache) => match cache.action {
                    CacheAction::Clear(_) => {}
                    CacheAction::Load(_) => {}
                },
            },
        }

        Ok(())
    }
}
