//! Module that handles the interaction with the Riot API.

use anyhow::Result;
use riven::{RiotApi, RiotApiConfig};
use std::env;

pub mod account;
pub mod mastery;
pub mod ranked;

pub trait ApiInstanceGetter {
    /// Returns a new instance of the Riot API.
    fn get_api_instance() -> Result<RiotApi>;
}

impl ApiInstanceGetter for RiotApi {
    fn get_api_instance() -> Result<RiotApi> {
        let api_key = env::var("RIOT_API_KEY")?;
        println!("API Key: {}", api_key);
        Ok(RiotApi::new(RiotApiConfig::with_key(api_key)))
    }
}
