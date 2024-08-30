//! Rank module

use crate::config::{Config, Image, Mode};
use riven::{
    consts::{Division, PlatformRoute, QueueType, Tier},
    models::{league_v4::LeagueEntry, summoner_v4::Summoner},
    RiotApi,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RetrieverError {
    #[error("No rank found in queue {0:?}")]
    NoRankFound(QueueType),

    #[error("Failed to fetch league entries for summoner")]
    FetchError(#[from] riven::RiotApiError),
}

trait Retriever {
    /// Returns the rank of a summoner.
    async fn get_rank(
        &self,
        summonner: &Summoner,
        route: PlatformRoute,
        queue: QueueType,
    ) -> Result<RankedInfo, RetrieverError>;
}

impl Retriever for RiotApi {
    async fn get_rank(
        &self,
        summonner: &Summoner,
        route: PlatformRoute,
        queue: QueueType,
    ) -> Result<RankedInfo, RetrieverError> {
        let entries = self
            .league_v4()
            .get_league_entries_for_summoner(route, &summonner.id)
            .await?;

        for entry in entries {
            if entry.queue_type == queue {
                return RankedInfo::from_entry(entry).ok_or(RetrieverError::NoRankFound(queue));
            }
        }

        Err(RetrieverError::NoRankFound(queue))
    }
}

#[derive(Debug)]
pub struct RankedInfo {
    pub queue: QueueType,
    pub tier: Tier,
    pub division: Option<Division>,
    pub lp: i32,
    pub wins: i32,
    pub losses: i32,
}

impl RankedInfo {
    /// Tries to create a `RankedInfo` from a `LeagueEntry`.
    /// Returns `None` if the queue type is not `RANKED_SOLO_5x5` or `RANKED_FLEX_SR`.
    fn from_entry(entry: LeagueEntry) -> Option<Self> {
        match entry.queue_type {
            QueueType::RANKED_SOLO_5x5 | QueueType::RANKED_FLEX_SR => {}
            _ => return None,
        }
        Some(Self {
            queue: entry.queue_type,
            tier: entry.tier.unwrap_or(Tier::UNRANKED),
            division: entry.rank,
            lp: entry.league_points,
            wins: entry.wins,
            losses: entry.losses,
        })
    }
}

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error("{0}")]
    FetchError(#[from] RetrieverError),
}

pub trait Fetcher {
    /// Fetches the rank of a summoner.
    async fn fetch_rank(
        &self,
        summonner: &Summoner,
        queue: QueueType,
        config: &Config,
    ) -> Result<Option<RankedInfo>, FetcherError>;
}

impl Fetcher for RiotApi {
    async fn fetch_rank(
        &self,
        summonner: &Summoner,
        queue: QueueType,
        config: &Config,
    ) -> Result<Option<RankedInfo>, FetcherError> {
        if matches!(config.mode, Mode::Ranked(_)) || matches!(config.image, Image::RankIcon) {
            Ok(Some(
                self.get_rank(summonner, config.account.server, queue)
                    .await?,
            ))
        } else {
            Ok(None)
        }
    }
}
