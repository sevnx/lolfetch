use crate::{
    api::{
        self,
        account::{self, Fetcher as AccountFetcher, PuuidFetchError},
        Fetcher,
    },
    cache,
    cli::{self, cache::CacheAction, Cli, Commands},
    config::{Config, Lolfetch},
    data::ApplicationData,
    display::Layout,
    logging,
};
use anyhow::Result;
use clap::Parser;
use riven::{RiotApi, RiotApiConfig};

pub struct App {}

impl App {
    pub async fn run(cli: Cli) -> Result<()> {
        if cli.verbose {
            logging::setup();
        }
        info!("Starting lolfetch");

        let api = RiotApi::new(RiotApiConfig::with_key(&cli.api_key));

        match cli.command {
            Commands::Display(config) => handle_display(&api, config).await,
            Commands::Cache(cache) => handle_cache(&api, cache).await,
        }
    }
}

async fn handle_display(api: &RiotApi, config: cli::lolfetch::Lolfetch) -> Result<()> {
    let config = Config::from_cli(config)?;
    let data = api.fetch(&config).await?;
    let processed = ApplicationData::process(data, &config).await?;
    info!("Displaying data");
    Layout::new(processed).display()?;
    Ok(())
}

async fn handle_cache(api: &RiotApi, config: cli::cache::Cache) -> Result<()> {
    match config.action {
        CacheAction::Clear(config) => match config.summoner {
            Some(summoner_config) => {
                match api.fetch_summoner(&summoner_config.clone().into()).await {
                    Ok(summoner) => {
                        cache::Cache::clear(Some((summoner, summoner_config.server.into())))
                    }
                    Err(e) => match e {
                        account::FetcherError::PuuidError(PuuidFetchError::AccountNotFound) => {
                            anyhow::bail!("Riot ID not found");
                        }
                        account::FetcherError::SummonerNotFound => {
                            anyhow::bail!("The summoner was found but not on the given server")
                        }
                        account::FetcherError::PuuidError(PuuidFetchError::ApiError(e))
                        | account::FetcherError::FetchError(e) => {
                            anyhow::bail!("Error fetching account: {e}");
                        }
                    },
                }
            }
            None => cache::Cache::clear(None),
        },
        CacheAction::Load(_) => {
            todo!()
        }
    }
}
