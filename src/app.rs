use anyhow::Result;
use clap::Parser;
use riven::{RiotApi, RiotApiConfig};

use crate::{
    api::{account::Fetcher as AccountFetcher, Fetcher},
    cache,
    cli::{self, cache::CacheAction, Cli, Commands},
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
        info!("Starting lolfetch");

        let api = RiotApi::new(RiotApiConfig::with_key(&options.api_key));

        match options.command {
            Commands::Display(config) => {
                let config = Config::from_cli(config)?;
                let data = api.fetch(&config).await?;
                let processed = ApplicationData::process(data, &config).await?;
                info!("Displaying data");
                Layout::new(processed).display()?;
            }
            Commands::Cache(cache) => match cache.action {
                CacheAction::Clear(config) => match config.summoner {
                    Some(summoner_config) => {
                        if let Ok(summoner) =
                            api.fetch_summoner(&summoner_config.clone().into()).await
                        {
                            cache::Cache::clear(Some((summoner, summoner_config.server.into())));
                        } else {
                            error!("Failed to fetch summoner {summoner_config:?}");
                        }
                    }
                    None => {
                        cache::Cache::clear(None);
                    }
                },
                CacheAction::Load(_) => {}
            },
        }

        Ok(())
    }
}
