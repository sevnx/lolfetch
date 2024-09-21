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
        let mut cache =
            cache::Cache::load_cache_from_file(summoner.clone(), config.account.server)?;

        // Ranked information.
        let ranked = self
            .fetch_rank(&summoner, QueueType::RANKED_SOLO_5x5, config)
            .await?;

        // TODO: Handle error better
        let criteria = config
            .mode
            .to_match_criteria()
            .ok_or(anyhow::anyhow!("Invalid mode for fetching matches"))?;

        let matches = match self
            .fetch_recent_matches(
                &summoner,
                config.account.server.to_regional(),
                &cache,
                &criteria,
            )
            .await?
        {
            Some(matches) => matches,
            None => {
                anyhow::bail!("The summoner does not have any matches to display");
            }
        };

        let masteries = self
            .fetch_mastery(&summoner, config.account.server, &config.mode)
            .await?;

        for info in matches {
            match cache.insert(info.id.clone(), info).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to insert match into cache: {e:?}");
                }
            }
        }

        let matches = cache.save_to_file()?;

        info!("Data fetched successfully");

        let image_url = match config.image.clone() {
            Image::Default => match config.mode {
                Mode::Ranked(_) | Mode::Lolfetch(_) => match ranked.as_ref() {
                    Some(ranked) => match ranked.tier {
                        Some(tier) => tier.get_icon_url().await,
                        None => anyhow::bail!("No tier found"),
                    },
                    None => anyhow::bail!("No ranked data found"),
                },
                Mode::Mastery(_) => {
                    masteries
                        .as_ref()
                        .expect("Masteries should be fetched")
                        .first()
                        .expect("There should be at least one mastery")
                        .champion_id
                        .get_icon_url()
                        .await
                }
                Mode::RecentMatches(_) => {
                    matches
                        .first()
                        .expect("There should be at least one match")
                        .info
                        .participants
                        .iter()
                        .find(|p| p.puuid == summoner.puuid)
                        .expect("Summoner should be in the match")
                        .champion()
                        .expect("Champion should be found")
                        .get_icon_url()
                        .await
                }
                Mode::Custom(_) => anyhow::bail!("Custom mode cannot use default image"),
            },
            Image::RankIcon => match ranked.as_ref() {
                Some(ranked) => match ranked.tier {
                    Some(tier) => tier.get_icon_url().await,
                    None => anyhow::bail!("No tier found when ranked icon was requested"),
                },
                None => anyhow::bail!("No ranked data found when ranked icon was requested"),
            },
            Image::ChampionIcon(champ) => champ.get_icon_url().await,
            Image::SummonerIcon => summoner.get_icon_url().await,
            Image::Custom(url) => url,
        };

        Ok(Data {
            summoner,
            ranked,
            matches: Some(matches),
            masteries,
            image_url,
        })
    }
}
