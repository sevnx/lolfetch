//! League of Legends match data.

use crate::{
    cache,
    config::{self, Mode},
    models::matches::MatchInfo,
};
use riven::{
    consts::{Queue, RegionalRoute},
    models::{match_v5::Match, summoner_v4::Summoner},
    RiotApi,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdRetrieverError {
    #[error("Failed to fetch matches: {0}")]
    Fetch(#[from] riven::RiotApiError),
}

trait Retriever {
    /// Returns x recent match IDs of a summonner
    async fn get_recent_matches_ids(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        count: i32,
        queue: Option<Queue>,
    ) -> Result<Vec<String>, riven::RiotApiError>;
}

impl Retriever for RiotApi {
    async fn get_recent_matches_ids(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        count: i32,
        queue: Option<Queue>,
    ) -> Result<Vec<String>, riven::RiotApiError> {
        self.match_v5()
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
            .await
    }
}

fn is_remake(match_data: &Match) -> bool {
    const MINUTES_UNTIL_REMAKE: i64 = 3;
    match_data.info.game_duration < MINUTES_UNTIL_REMAKE * 60
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
            Mode::Lolfetch(ref lolfetch) => Some(MatchCriteria {
                count: lolfetch.games,
                queue: None,
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
    FetchError(#[from] riven::RiotApiError),

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
        cache: &cache::Cache,
    ) -> Result<Option<Vec<MatchInfo>>, FetcherError>;
}

impl Fetcher for RiotApi {
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        config: &config::Mode,
        cache: &cache::Cache,
    ) -> Result<Option<Vec<MatchInfo>>, FetcherError> {
        let criteria = config
            .to_match_criteria()
            .ok_or(FetcherError::InvalidMode)?;

        let ids = self
            .get_recent_matches_ids(summoner, route, criteria.count, criteria.queue)
            .await?;

        let mut matches = Vec::new();

        for id in ids {
            if !cache.contains(id.clone()) {
                let match_info = self.match_v5().get_match(route, &id).await?.unwrap();
                let timeline = self.match_v5().get_timeline(route, &id).await?.unwrap();

                let info = MatchInfo {
                    id: match_info.metadata.match_id.clone(),
                    match_info: match_info.info,
                    timeline_info: Some(timeline.info),
                };

                matches.push(info);
            } else {
                println!("Ignoring match {id}");
            }
        }

        Ok(Some(matches))
    }
}
