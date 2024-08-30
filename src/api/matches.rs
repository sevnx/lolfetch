//! League of Legends match data.

use crate::config::{self, Mode};
use riven::{
    consts::{Queue, RegionalRoute},
    models::{match_v5::Match, summoner_v4::Summoner},
    RiotApi,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RetrieverError {
    #[error("Failed to fetch matches: {0}")]
    FetchError(#[from] riven::RiotApiError),

    #[error("Failed to get match data")]
    MatchDataError,
}

trait Retriever {
    /// Returns x recent matches of a summoner.
    async fn get_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        count: i32,
        queue: Option<Queue>,
    ) -> Result<Vec<Match>, RetrieverError>;
}

impl Retriever for RiotApi {
    async fn get_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        count: i32,
        queue: Option<Queue>,
    ) -> Result<Vec<Match>, RetrieverError> {
        let match_list = self
            .match_v5()
            .get_match_ids_by_puuid(
                route,
                &summoner.puuid,
                Some(count),
                None,
                queue,
                None,
                None,
                None,
            )
            .await?;

        let mut matches = Vec::new();
        for match_id in match_list {
            let match_data = self
                .match_v5()
                .get_match(route, &match_id)
                .await?
                .ok_or_else(|| RetrieverError::MatchDataError)?;
            matches.push(match_data);
        }
        Ok(matches)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatchCriteria {
    pub count: i32,
    pub queue: Option<Queue>,
}

impl Mode {
    fn to_match_criteria(&self) -> Option<MatchCriteria> {
        match self {
            Mode::Ranked(ref ranked) => Some(MatchCriteria {
                count: ranked.games,
                queue: Some(Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO),
            }),
            Mode::Mastery(ref mastery) => Some(MatchCriteria {
                count: mastery.games,
                queue: None,
            }),
            Mode::RecentMatches(ref recent_matches) => Some(MatchCriteria {
                count: recent_matches.recent_matches,
                queue: None,
            }),
            Mode::Custom(_) => None,
        }
    }
}

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("Failed to retrieve matches: {0}")]
    FetchError(#[from] RetrieverError),

    #[error("Invalid mode for fetching matches")]
    InvalidMode,
}

pub trait Fetcher {
    /// Fetches the recent matches of a summoner.
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        config: &config::Mode,
    ) -> Result<Option<Vec<Match>>, FetcherError>;
}

impl Fetcher for RiotApi {
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        config: &config::Mode,
    ) -> Result<Option<Vec<Match>>, FetcherError> {
        let criteria = config
            .to_match_criteria()
            .ok_or(FetcherError::InvalidMode)?;

        self.get_recent_matches(summoner, route, criteria.count, criteria.queue)
            .await
            .map(Some)
            .map_err(FetcherError::FetchError)
    }
}
