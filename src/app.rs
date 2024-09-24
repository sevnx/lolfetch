use crate::{
    api::{
        account::{self, Fetcher as AccountFetcher, PuuidFetchError},
        matches::{Fetcher, MatchCriteria},
        Fetcher as ApiFetcher,
    },
    cache::{self, CacheSaveOptions},
    cli::{self, cache::CacheAction, Cli, Commands},
    config::Config,
    data::ApplicationData,
    display::Layout,
    logging,
};
use anyhow::Result;
use riven::{consts::Queue, RiotApi, RiotApiConfig};

pub struct App {}

impl App {
    pub async fn run(cli: Cli) -> Result<()> {
        // Initialize logging
        if cli.verbose {
            match logging::setup() {
                Ok(()) => info!("Logging initialized"),
                Err(e) => eprintln!("Error initializing logging: {e}"),
            }
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
        CacheAction::Clear(config) => handle_cache_clear(api, config).await,
        CacheAction::Load(config) => handle_cache_load(api, config).await,
    }
}

async fn handle_cache_clear(api: &RiotApi, config: cli::cache::Clear) -> Result<()> {
    match config.summoner {
        Some(summoner_config) => match api.fetch_summoner(&summoner_config.clone().into()).await {
            Ok(summoner) => cache::Cache::clear(Some((summoner, summoner_config.server.into()))),
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
        },
        None => cache::Cache::clear(None),
    }
}

async fn handle_cache_load(api: &RiotApi, config: cli::cache::Load) -> Result<()> {
    const MAX_MATCHES_PER_REQUEST: i32 = 100;

    let account = config.summoner.clone().into();
    let summoner = api.fetch_summoner(&account).await?;

    let mut cache =
        cache::Cache::load_cache_from_file(summoner.clone(), config.summoner.server.into())?;

    let mut count = config.matches;

    // This isn't that accurate because remakes are not counted but it's good enough
    let mut start_at: i32 = cache.len() as i32;

    'game: loop {
        let matches_query = match count {
            Some(ref mut c) => {
                let current_count = *c;
                let min = if current_count < MAX_MATCHES_PER_REQUEST {
                    current_count
                } else {
                    MAX_MATCHES_PER_REQUEST
                };
                *c -= min;
                min
            }
            None => MAX_MATCHES_PER_REQUEST,
        };

        let match_criteria = MatchCriteria {
            count: matches_query,
            queue: Some(Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO),
            start_at: Some(start_at),
        };

        // Fetch match ids
        let matches = api
            .fetch_recent_matches(
                &summoner,
                account.server.to_regional(),
                &cache,
                &match_criteria,
            )
            .await?;

        // Insert matches into cache
        match matches {
            Some(matches) => {
                for info in matches {
                    let id = info.id.clone();
                    match cache.insert(id.clone(), info).await {
                        Ok(()) => info!("Inserted match {id}"),
                        Err(e) => match e {
                            cache::CacheInsertError::AlreadyExists => {
                                warn!("Match {id} already exists in cache");
                            }
                            cache::CacheInsertError::Remake => {
                                warn!("Match {id} is a remake");
                            }
                            cache::CacheInsertError::PatchMismatch => {
                                warn!("Match {id} is from a different patch");
                                // This means that we are not in the same season, we can stop here
                                break 'game;
                            }
                        },
                    }
                }
            }
            // No games found
            None => break,
        }

        start_at += matches_query;
    }

    // Save cache to file
    cache.save(CacheSaveOptions::from_bool(!config.no_save))?;

    Ok(())
}
