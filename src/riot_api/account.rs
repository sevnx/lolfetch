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
            return Err(anyhow::anyhow!("Invalid Riot ID"));
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

#[cfg(test)]
mod tests {
    use crate::*;
    use riot_api::{
        account::{AccountFetchError, PuuidGetter, RiotId},
        ApiInstanceGetter,
    };

    #[test]
    fn test_good_riot_id_from_string() {
        let riot_id = RiotId::from_str("username#1234");
        match riot_id {
            Ok(riot_id) => {
                assert_eq!(riot_id.username, "username");
                assert_eq!(riot_id.tagline, "1234");
            }
            Err(_) => panic!("Failed to parse Riot ID"),
        }
    }

    #[test]
    fn test_bad_riot_id_from_string() {
        let riot_id = RiotId::from_str("username");
        assert!(!riot_id.is_ok(), "Parsed invalid Riot ID")
    }

    #[tokio_macros::test()]
    async fn test_get_riot_account_puuid_by_riot_id() {
        dotenv::dotenv().unwrap();

        let riot_id = RiotId::new("sev", "cat");
        let puuid_expected =
            "zcHuUSLP3G4GhB1BV0kcovyPEopvE4MJKBR22fjHl3HyxYdtLUEQpy8Mgf1-tLdVY-4vqoSMWfJ1Vg";

        let riot_api = riven::RiotApi::get_api_instance().unwrap();

        match riot_api.get_puuid(&riot_id).await {
            Ok(puuid) => assert_eq!(puuid, puuid_expected),
            Err(e) => match e {
                AccountFetchError::AccountNotFound => panic!("Account not found"),
                AccountFetchError::ApiError(e) => panic!("API error: {e}"),
            },
        }
    }
}
