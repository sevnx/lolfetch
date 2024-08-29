//! League of Legends match data.

use anyhow::Result;
use riven::{
    consts::{Queue, RegionalRoute},
    models::{match_v5::Match, summoner_v4::Summoner},
    RiotApi,
};

use crate::config::{self, Mode};

pub trait MatchGetter {
    /// Returns x recent matches of a summoner.
    async fn get_recent_matches(
        &self,
        api: &RiotApi,
        route: RegionalRoute,
        count: i32,
        queue: Option<Queue>,
    ) -> Result<Vec<Match>>;
}

impl MatchGetter for Summoner {
    async fn get_recent_matches(
        &self,
        api: &RiotApi,
        route: RegionalRoute,
        count: i32,
        queue: Option<Queue>,
    ) -> Result<Vec<Match>> {
        let match_list = api
            .match_v5()
            .get_match_ids_by_puuid(
                route,
                &self.puuid,
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
            let match_data = api
                .match_v5()
                .get_match(route, &match_id)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Match with ID {} not found", match_id))?;
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

pub trait MatchFetcher {
    /// Fetches the recent matches of a summoner.
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        config: &config::Mode,
    ) -> Result<Option<Vec<Match>>>;
}

impl MatchFetcher for RiotApi {
    async fn fetch_recent_matches(
        &self,
        summoner: &Summoner,
        route: RegionalRoute,
        config: &config::Mode,
    ) -> Result<Option<Vec<Match>>> {
        let criteria = config.to_match_criteria().ok_or(anyhow::anyhow!(
            "Invalid mode for fetching matches: {:?}",
            config
        ))?;

        summoner
            .get_recent_matches(self, route, criteria.count, criteria.queue)
            .await
            .map(Some)
    }
}
