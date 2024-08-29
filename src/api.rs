//! Module that handles the interaction with the various APIs used to gather data.

use account::AccountFetcher;
use matches::{MatchFetcher, MatchGetter};
use rank::{RankFetcher, RankRetriever};
use riven::{
    consts::{Queue, QueueType},
    models::{champion_mastery_v4, match_v5},
    RiotApi,
};
use static_data::IconGetter;

use crate::{
    config::{Config, Image, Mode},
    display::utils::ImageUrlGetter,
};

pub mod account;
pub mod mastery;
pub mod matches;
pub mod rank;
pub mod region;
pub mod static_data;

/// Extension trait to fetch data from the API.
pub trait Fetcher {
    async fn fetch(&self, config: &Config) -> Result<Data>;
}

/// Data struct to holds the various data fetched from the API.
/// Each field is linked to specific data / module.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct Data {
    /// Ranked information.
    pub ranked: Option<rank::RankedInfo>,
    /// Matches.
    pub matches: Option<Vec<match_v5::Match>>,
    /// Champion masteries.
    pub masteries: Option<Vec<champion_mastery_v4::ChampionMastery>>,
    /// Image URL.
    pub image_url: Option<String>,
}

// TODO: Clean up
use anyhow::Result;

impl Fetcher for RiotApi {
    async fn fetch(&self, config: &Config) -> Result<Data> {
        // Construct commonly used data structs for fetching data.
        let summoner = self.fetch_summoner(&config.account).await?;

        // Ranked information.
        let ranked = self
            .fetch_rank(&summoner, QueueType::RANKED_SOLO_5x5, config)
            .await?;

        let matches = self
        .fetch_recent_matches(&summoner, config.account.server.to_regional(), &config.mode)
            .await?;

        // Image URL.
        let image = match config.image.clone() {
            Image::Default => None,
            Image::RankIcon => Some(ranked.as_ref().unwrap().tier.get_image_url()),
            Image::ChampionIcon(champ) => Some(champ.get_icon_url().await),
            Image::SummonerIcon => Some(summoner.get_icon_url().await),
            Image::Custom(url) => Some(url),
        };

        Ok(Data {
            ranked,
            matches,
            masteries: None,
            image_url: image,
        })
    }
}
