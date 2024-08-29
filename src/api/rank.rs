//! Rank module

use anyhow::Result;
use riven::{
    consts::{Division, PlatformRoute, QueueType, Tier},
    models::{league_v4::LeagueEntry, summoner_v4::Summoner},
    RiotApi,
};

pub trait RankRetriever {
    /// Returns the rank of a summoner.
    async fn get_rank(
        &self,
        api: &RiotApi,
        route: PlatformRoute,
        queue: QueueType,
    ) -> Result<RankedInfo>;
}

impl RankRetriever for Summoner {
    async fn get_rank(
        &self,
        api: &RiotApi,
        route: PlatformRoute,
        queue: QueueType,
    ) -> Result<RankedInfo> {
        let entries = api
            .league_v4()
            .get_league_entries_for_summoner(route, &self.id)
            .await?;

        for entry in entries {
            if entry.queue_type == queue {
                return RankedInfo::from_entry(entry);
            }
        }

        Err(anyhow::anyhow!(format!(
            "No rank found in queue {:?}",
            queue
        )))
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
    fn from_entry(entry: LeagueEntry) -> Result<Self> {
        match entry.queue_type {
            QueueType::RANKED_SOLO_5x5 | QueueType::RANKED_FLEX_SR => {}
            _ => return Err(anyhow::anyhow!("Invalid queue type")),
        }
        Ok(Self {
            queue: entry.queue_type,
            tier: entry.tier.unwrap_or(Tier::UNRANKED),
            division: entry.rank,
            lp: entry.league_points,
            wins: entry.wins,
            losses: entry.losses,
        })
    }
}
