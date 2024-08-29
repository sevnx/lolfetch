//! Module that handles the interaction with the various APIs used to gather data.

use account::PuuidGetter;
use matches::MatchGetter;
use rank::RankRetriever;
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
        let route = config.account.server.into();
        let summonner = self
            .summoner_v4()
            .get_by_puuid(route, &self.get_puuid(&config.account.riot_id).await?)
            .await?;

        // Ranked information.
        let ranked = match config.mode {
            Mode::Ranked(_) => Some(
                summonner
                    .get_rank(self, route, QueueType::RANKED_SOLO_5x5)
                    .await?,
            ),
            _ => match config.image {
                Image::RankIcon => Some(
                    summonner
                        .get_rank(self, route, QueueType::RANKED_SOLO_5x5)
                        .await?,
                ),
                _ => None,
            },
        };

        // Game fetcher.
        let (fetched_games, queue) = match config.mode {
            Mode::Ranked(ref ranked) => (
                Some(ranked.games),
                Some(Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO),
            ),
            Mode::Mastery(ref mastery) => (Some(mastery.games), None),
            Mode::RecentMatches(ref recent_matches) => (Some(recent_matches.recent_matches), None),
            Mode::Custom(_) => (None, None),
        };
        let matches = match fetched_games {
            Some(games) => Some(
                summonner
                    .get_recent_matches(self, route.to_regional(), games, queue)
                    .await?,
            ),
            None => None,
        };

        // Image URL.
        let image = match config.image.clone() {
            Image::Default => None,
            Image::RankIcon => Some(ranked.as_ref().unwrap().tier.get_image_url()),
            Image::ChampionIcon(champ) => Some(champ.get_icon_url().await),
            Image::SummonerIcon => Some(summonner.get_icon_url().await),
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
