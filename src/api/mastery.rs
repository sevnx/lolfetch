use riven::{
    consts::PlatformRoute,
    models::{champion_mastery_v4::ChampionMastery, summoner_v4::Summoner},
    RiotApi,
};
use thiserror::Error;

use crate::config;

#[derive(Error, Debug)]
pub enum RetrieverError {
    #[error("Failed to fetch mastery: {0}")]
    FetchError(#[from] riven::RiotApiError),
}

/// Retrieves champion masteries.
trait Retriever {
    /// Returns X champion masteries of a summoner.
    async fn get_mastery(
        &self,
        summoner: &Summoner,
        route: PlatformRoute,
        count: i32,
    ) -> Result<Vec<ChampionMastery>, RetrieverError>;
}

impl Retriever for RiotApi {
    async fn get_mastery(
        &self,
        summoner: &Summoner,
        route: PlatformRoute,
        count: i32,
    ) -> Result<Vec<ChampionMastery>, RetrieverError> {
        self.champion_mastery_v4()
            .get_top_champion_masteries_by_puuid(route, &summoner.id, Some(count))
            .await
            .map_err(RetrieverError::FetchError)
    }
}

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("{0}")]
    FetchError(#[from] RetrieverError),
}

pub trait Fetcher {
    /// Fetches X champion masteries of a summoner.
    async fn fetch_mastery(
        &self,
        summoner: &Summoner,
        route: PlatformRoute,
        mode: &config::Mode,
    ) -> Result<Option<Vec<ChampionMastery>>, FetcherError>;
}

impl Fetcher for RiotApi {
    async fn fetch_mastery(
        &self,
        summoner: &Summoner,
        route: PlatformRoute,
        mode: &config::Mode,
    ) -> Result<Option<Vec<ChampionMastery>>, FetcherError> {
        match mode {
            config::Mode::Mastery(ref mastery) => {
                let count = mastery.games;
                self.get_mastery(summoner, route, count)
                    .await
                    .map(Some)
                    .map_err(FetcherError::FetchError)
            }
            _ => Ok(None),
        }
    }
}
