use anyhow::Result;
use riven::{
    consts::PlatformRoute,
    models::{champion_mastery_v4::ChampionMastery, summoner_v4::Summoner},
    RiotApi,
};

pub trait MasteryRetriever {
    /// Returns X champion masteries of a summoner.
    async fn get_mastery(
        &self,
        api: &RiotApi,
        route: PlatformRoute,
        count: i32,
    ) -> Result<Vec<ChampionMastery>>;
}

impl MasteryRetriever for Summoner {
    async fn get_mastery(
        &self,
        api: &RiotApi,
        route: PlatformRoute,
        count: i32,
    ) -> Result<Vec<ChampionMastery>> {
        let masteries = api
            .champion_mastery_v4()
            .get_top_champion_masteries_by_puuid(route, &self.puuid, Some(count))
            .await?;
        Ok(masteries)
    }
}
