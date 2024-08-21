//! League of Legends match data.

use anyhow::Result;
use riven::{
    consts::{Queue, RegionalRoute},
    models::{match_v5::Match, summoner_v4::Summoner},
    RiotApi,
};

pub trait MatchGetter {
    /// Returns x recent matches of a summoner.
    async fn get_recent_matches(
        &self,
        api: &RiotApi,
        route: RegionalRoute,
        count: i32,
        queue: Queue,
    ) -> Result<Vec<Match>>;
}

impl MatchGetter for Summoner {
    async fn get_recent_matches(
        &self,
        api: &RiotApi,
        route: RegionalRoute,
        count: i32,
        queue: Queue,
    ) -> Result<Vec<Match>> {
        let match_list = api
            .match_v5()
            .get_match_ids_by_puuid(
                route,
                &self.puuid,
                Some(count),
                None,
                Some(queue),
                None,
                None,
                None,
            )
            .await?;

        let mut matches = Vec::new();
        for match_id in match_list {
            let match_data = api.match_v5().get_match(route, &match_id).await?.unwrap();
            matches.push(match_data);
        }
        Ok(matches)
    }
}
