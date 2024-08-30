//! Rank module

use crate::config::{Config, Image, Mode};
use riven::{
    consts::{PlatformRoute, QueueType},
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
    ) -> Result<LeagueEntry, RetrieverError>;
}

impl Retriever for RiotApi {
    async fn get_rank(
        &self,
        summonner: &Summoner,
        route: PlatformRoute,
        queue: QueueType,
    ) -> Result<LeagueEntry, RetrieverError> {
        let entries = self
            .league_v4()
            .get_league_entries_for_summoner(route, &summonner.id)
            .await?;

        for entry in entries {
            if entry.queue_type == queue {
                return Ok(entry);
            }
        }

        Err(RetrieverError::NoRankFound(queue))
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
    ) -> Result<Option<LeagueEntry>, FetcherError>;
}

impl Fetcher for RiotApi {
    async fn fetch_rank(
        &self,
        summonner: &Summoner,
        queue: QueueType,
        config: &Config,
    ) -> Result<Option<LeagueEntry>, FetcherError> {
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
