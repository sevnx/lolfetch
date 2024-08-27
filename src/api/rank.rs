//! Rank module

use anyhow::Result;
use riven::{
    consts::PlatformRoute,
    models::{league_v4::LeagueEntry, summoner_v4::Summoner},
    RiotApi,
};

pub trait RankRetriever {
    /// Returns the rank of a summoner.
    async fn get_rank(&self, api: &RiotApi, route: PlatformRoute) -> Result<LeagueEntry>;
}

impl RankRetriever for Summoner {
    async fn get_rank(&self, api: &RiotApi, route: PlatformRoute) -> Result<LeagueEntry> {
        let entries = api
            .league_v4()
            .get_league_entries_for_summoner(route, &self.id)
            .await?;

        for entry in entries {
            if entry.queue_type == riven::consts::QueueType::RANKED_SOLO_5x5 {
                return Ok(entry);
            }
        }

        Err(anyhow::anyhow!("No ranked solo 5x5 entry found"))
    }
}
