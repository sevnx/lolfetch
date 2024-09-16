//! Module that handles the interaction with the various APIs used to gather data.

use crate::{
    cache,
    config::{Config, Image, Mode},
    models::matches::MatchInfo,
};
use account::Fetcher as AccountFetcher;
use anyhow::Result;
use mastery::Fetcher as MasteryFetcher;
use matches::Fetcher as MatchesFetcher;
use rank::Fetcher as RankFetcher;
use riven::{
    consts::QueueType,
    models::{champion_mastery_v4, league_v4, summoner_v4},
    RiotApi,
};
use tooling::static_data::IconGetter;

pub mod account;
pub mod mastery;
pub mod matches;
pub mod rank;
pub mod tooling;

/// Extension trait to fetch data from the API.
pub trait Fetcher {
    async fn fetch(&self, config: &Config) -> Result<Data>;
}

/// Data struct to holds the various data fetched from the API.
/// Each field is linked to specific data / module.
#[non_exhaustive]
#[derive(Debug)]
pub struct Data {
    /// Summoner information.
    pub summoner: summoner_v4::Summoner,
    /// Ranked information.
    pub ranked: Option<league_v4::LeagueEntry>,
    /// Matches.
    pub matches: Option<Vec<MatchInfo>>,
    /// Champion masteries.
    pub masteries: Option<Vec<champion_mastery_v4::ChampionMastery>>,
    /// Image URL.
    pub image_url: String,
}

// TODO: Clean up

impl Fetcher for RiotApi {
    async fn fetch(&self, config: &Config) -> Result<Data> {
        info!("Fetching data from Riot API");

        // Construct commonly used data structs for fetching data.
        let summoner = self.fetch_summoner(&config.account).await?;

        // Get cached data
        let mut cache = cache::Cache::load_cache(summoner.clone(), config.account.server)?;

        // Ranked information.
        let ranked = self
            .fetch_rank(&summoner, QueueType::RANKED_SOLO_5x5, config)
            .await?;

        let matches = self
            .fetch_recent_matches(
                &summoner,
                config.account.server.to_regional(),
                &config.mode,
                &cache,
            )
            .await?;

        let masteries = self
            .fetch_mastery(&summoner, config.account.server, &config.mode)
            .await?;

        // TODO: CLEAN THIS UP
        let image_url = match config.image.clone() {
            Image::Default => match config.mode {
                Mode::Ranked(_) | Mode::Lolfetch(_) => {
                    ranked.as_ref().unwrap().tier.unwrap().get_icon_url().await
                }
                Mode::Mastery(_) => {
                    masteries
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .champion_id
                        .get_icon_url()
                        .await
                }
                Mode::RecentMatches(_) => {
                    matches
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .info
                        .participants
                        .iter()
                        .find(|p| p.puuid == summoner.puuid)
                        .unwrap()
                        .champion()
                        .unwrap()
                        .get_icon_url()
                        .await
                }
                Mode::Custom(_) => anyhow::bail!("Custom mode cannot use default image"),
            },
            Image::RankIcon => ranked.as_ref().unwrap().tier.unwrap().get_icon_url().await,
            Image::ChampionIcon(champ) => champ.get_icon_url().await,
            Image::SummonerIcon => summoner.get_icon_url().await,
            Image::Custom(url) => url,
        };

        if let Some(matches) = matches {
            matches
                .into_iter()
                .for_each(|info| cache.insert(info.id.clone(), info));
        }

        let matches = cache.save()?;

        info!("Data fetched successfully");

        Ok(Data {
            summoner,
            ranked,
            matches: Some(matches),
            masteries,
            image_url,
        })
    }
}
