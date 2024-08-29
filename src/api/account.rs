//! Accounts

use riven::{consts::RegionalRoute, models::summoner_v4};
use thiserror::Error as ThisError;

use crate::config;

/// `RiotId`, used to identify a user in the Riot API.
#[derive(Debug, Clone)]
pub struct RiotId {
    pub username: String,
    pub tagline: String,
}

impl RiotId {
    pub fn new(username: impl Into<String>, tagline: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            tagline: tagline.into(),
        }
    }

    /// Creates a `RiotId` from a string.
    pub fn from_str(riot_id: &str) -> anyhow::Result<Self> {
        let mut split: Vec<&str> = riot_id.split('#').collect();
        if split.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid Riot ID, expected format: username#tag"
            ));
        }
        Ok(Self::new(split.remove(0), split.remove(0)))
    }
}

impl std::fmt::Display for RiotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", self.username, self.tagline)
    }
}

#[derive(ThisError, Debug)]
pub enum AccountFetchError {
    #[error("Account not found")]
    AccountNotFound,
    #[error("An API error was encountered")]
    ApiError(#[from] riven::RiotApiError),
}

trait PuuidGetter {
    /// Returns the PUUID of a user.
    /// Checks all regions for the user.
    async fn get_puuid(&self, riot_id: &RiotId) -> Result<String, AccountFetchError>;
}

impl PuuidGetter for riven::RiotApi {
    async fn get_puuid(&self, riot_id: &RiotId) -> Result<String, AccountFetchError> {
        match self
            .account_v1()
            .get_by_riot_id(RegionalRoute::EUROPE, &riot_id.username, &riot_id.tagline)
            .await
        {
            Ok(account) => account
                .map(|account| account.puuid)
                .ok_or(AccountFetchError::AccountNotFound),
            Err(e) => Err(AccountFetchError::ApiError(e)),
        }
    }
}

pub trait AccountFetcher {
    /// Fetches the summoner
    async fn fetch_summoner(
        &self,
        config: &config::Account,
    ) -> Result<summoner_v4::Summoner, AccountFetchError>;
}

impl AccountFetcher for riven::RiotApi {
    async fn fetch_summoner(
        &self,
        config: &config::Account,
    ) -> Result<summoner_v4::Summoner, AccountFetchError> {
        let route = config.server.into();
        self.summoner_v4()
            .get_by_puuid(route, &self.get_puuid(&config.riot_id).await?)
            .await
            .map_err(AccountFetchError::from)
    }
}
