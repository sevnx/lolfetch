//! Module that handles the interaction with the Riot API.

use anyhow::Result;
use riven::RiotApi;
use std::env;

mod account;

pub trait ApiInstanceGetter {
    /// Returns a new instance of the Riot API.
    fn get_api_instance() -> Result<RiotApi>;
}

impl ApiInstanceGetter for RiotApi {
    fn get_api_instance() -> Result<RiotApi> {
        let api_key = env::var("RIOT_API_KEY")?;
        Ok(RiotApi::new(api_key))
    }
}
