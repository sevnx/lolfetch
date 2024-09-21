//! League of Legends match data.

use crate::{cache, config::Mode, models::matches::MatchInfo};
use riven::{
    consts::{Queue, RegionalRoute},
    models::summoner_v4::Summoner,
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
        match_criteria: &MatchCriteria,
    ) -> Result<Vec<String>, riven::RiotApiError>;
}

impl Retriever for RiotApi {
    async fn get_recent_matches_ids(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        match_criteria: &MatchCriteria,
    ) -> Result<Vec<String>, riven::RiotApiError> {
        const MAX_MATCHES_PER_REQUEST: i32 = 100;

        let mut match_ids = Vec::new();

        let mut remaining = match_criteria.count;
        while remaining > 0 {
            let current_count = remaining.min(MAX_MATCHES_PER_REQUEST);

            let ids = self
                .match_v5()
                .get_match_ids_by_puuid(
                    route,
                    &summoner.puuid,
                    Some(current_count),
                    None,
                    match_criteria.queue,
                    None,
                    Some(match_ids.len() as i32 + match_criteria.start_at.unwrap_or(0)),
                    None,
                )
                .await?;

            match_ids.extend(ids);

            remaining -= current_count;
        }

        Ok(match_ids)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatchCriteria {
    pub count: i32,
    pub queue: Option<Queue>,
    pub start_at: Option<i32>,
}

impl Mode {
    pub const fn to_match_criteria(&self) -> Option<MatchCriteria> {
        match self {
            Self::Ranked(ref ranked) => Some(MatchCriteria {
                count: ranked.games,
                queue: Some(Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO),
                start_at: None,
            }),
            Self::Lolfetch(ref lolfetch) => Some(MatchCriteria {
                count: lolfetch.games,
                queue: None,
                start_at: None,
            }),
            Self::Mastery(ref mastery) => Some(MatchCriteria {
                count: mastery.games,
                queue: None,
                start_at: None,
            }),
            Self::RecentMatches(ref recent_matches) => Some(MatchCriteria {
                count: recent_matches.recent_matches,
                queue: None,
                start_at: None,
            }),
            Self::Custom(_) => None,
        }
    }
}

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("Failed to retrieve matches: {0}")]
    FetchError(#[from] riven::RiotApiError),
}

pub trait Fetcher {
    /// Fetches the recent matches of a summoner.
    /// Only fetches matches of the current patch
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        cache: &cache::Cache,
        criteria: &MatchCriteria,
    ) -> Result<Option<Vec<MatchInfo>>, FetcherError>;
}

impl Fetcher for RiotApi {
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        cache: &cache::Cache,
        criteria: &MatchCriteria,
    ) -> Result<Option<Vec<MatchInfo>>, FetcherError> {
        let ids = self
            .get_recent_matches_ids(summoner, route, criteria)
            .await?;

        let mut matches = Vec::new();

        for id in ids {
            if !cache.contains(&id) {
                info!("Fetching match {id}");

                let match_info = self.match_v5().get_match(route, &id).await?.unwrap();
                let timeline = self.match_v5().get_timeline(route, &id).await?.unwrap();

                let info = MatchInfo {
                    id: match_info.metadata.match_id.clone(),
                    info: match_info.info,
                    timeline: Some(timeline.info),
                };

                matches.push(info);
            } else {
                info!("Ignoring match {id} : cached");
            }
        }

        Ok(Some(matches))
    }
}
