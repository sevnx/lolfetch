//! Accounts

use riven::consts::RegionalRoute;
use thiserror::Error as ThisError;

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

pub trait PuuidGetter {
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
