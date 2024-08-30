//! API endpoints linked to account information.

use crate::config;
use riven::{consts::RegionalRoute, models::summoner_v4};
use std::fmt;
use thiserror::Error as ThisError;

/// `RiotId`, used to identify a user in the Riot API.
#[derive(Debug, Clone)]
pub struct RiotId {
    pub username: String,
    pub tagline: String,
}

#[derive(ThisError, Debug)]
pub enum RiotIdError {
    #[error("Invalid Riot ID, expected format: username#tag")]
    InvalidFormat,
}

impl RiotId {
    pub fn new(username: impl Into<String>, tagline: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            tagline: tagline.into(),
        }
    }

    /// Creates a `RiotId` from a string.
    pub fn from_str(riot_id: &str) -> Result<Self, RiotIdError> {
        let (username, tagline) = riot_id.split_once('#').ok_or(RiotIdError::InvalidFormat)?;
        Ok(Self::new(username, tagline))
    }
}

impl fmt::Display for RiotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", self.username, self.tagline)
    }
}

#[derive(ThisError, Debug)]
pub enum PuuidFetchError {
    #[error("Account not found")]
    AccountNotFound,
    #[error("An API error was encountered")]
    ApiError(#[from] riven::RiotApiError),
}

trait PuuidGetter {
    /// Returns the PUUID of a user.
    async fn get_puuid(&self, riot_id: &RiotId) -> Result<String, PuuidFetchError>;
}

impl PuuidGetter for riven::RiotApi {
    async fn get_puuid(&self, riot_id: &RiotId) -> Result<String, PuuidFetchError> {
        match self
            .account_v1()
            .get_by_riot_id(RegionalRoute::EUROPE, &riot_id.username, &riot_id.tagline)
            .await
        {
            Ok(account) => account
                .map(|account| account.puuid)
                .ok_or(PuuidFetchError::AccountNotFound),
            Err(e) => Err(PuuidFetchError::ApiError(e)),
        }
    }
}

#[derive(ThisError, Debug)]
pub enum FetcherError {
    #[error("{0}")]
    PuuidError(#[from] PuuidFetchError),

    #[error("Failed to fetch summoner")]
    FetchError(#[from] riven::RiotApiError),
}

pub trait Fetcher {
    /// Fetches the summoner
    async fn fetch_summoner(
        &self,
        config: &config::Account,
    ) -> Result<summoner_v4::Summoner, FetcherError>;
}

impl Fetcher for riven::RiotApi {
    async fn fetch_summoner(
        &self,
        config: &config::Account,
    ) -> Result<summoner_v4::Summoner, FetcherError> {
        let puuid = self.get_puuid(&config.riot_id).await?;

        self.summoner_v4()
            .get_by_puuid(config.server, &puuid)
            .await
            .map_err(FetcherError::FetchError)
    }
}
