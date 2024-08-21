//! Module that handles the interaction with the Riot API.

use anyhow::Result;
use riven::{
    consts::{PlatformRoute, RegionalRoute},
    RiotApi, RiotApiConfig,
};
use std::env;

use crate::cli::LeagueServer;

pub mod account;
pub mod mastery;
pub mod matches;
pub mod rank;

pub trait ApiInstanceGetter {
    /// Returns a new instance of the Riot API.
    fn get_api_instance() -> Result<RiotApi>;
}

impl ApiInstanceGetter for RiotApi {
    fn get_api_instance() -> Result<RiotApi> {
        let api_key = env::var("RIOT_API_KEY")?;
        Ok(RiotApi::new(RiotApiConfig::with_key(api_key)))
    }
}

/// Trait that converts a server to a regional route.
pub trait RegionFromServer {
    #[allow(dead_code)]
    fn from(&self) -> RegionalRoute;
}

impl RegionFromServer for PlatformRoute {
    fn from(&self) -> RegionalRoute {
        match self {
            PlatformRoute::BR1 => RegionalRoute::AMERICAS,
            PlatformRoute::EUN1 => RegionalRoute::EUROPE,
            PlatformRoute::EUW1 => RegionalRoute::EUROPE,
            PlatformRoute::JP1 => RegionalRoute::ASIA,
            PlatformRoute::KR => RegionalRoute::ASIA,
            PlatformRoute::LA1 => RegionalRoute::AMERICAS,
            PlatformRoute::LA2 => RegionalRoute::AMERICAS,
            PlatformRoute::NA1 => RegionalRoute::AMERICAS,
            PlatformRoute::OC1 => RegionalRoute::ASIA,
            PlatformRoute::RU => RegionalRoute::EUROPE,
            PlatformRoute::TR1 => RegionalRoute::EUROPE,
            PlatformRoute::SG2 => RegionalRoute::ASIA,
            PlatformRoute::PH2 => RegionalRoute::ASIA,
            PlatformRoute::VN2 => RegionalRoute::ASIA,
            PlatformRoute::TW2 => RegionalRoute::ASIA,
            PlatformRoute::TH2 => RegionalRoute::ASIA,
            PlatformRoute::ME1 => RegionalRoute::EUROPE,
            _ => unreachable!(),
        }
    }
}

impl From<LeagueServer> for PlatformRoute {
    fn from(server: LeagueServer) -> Self {
        match server {
            LeagueServer::Br => PlatformRoute::BR1,
            LeagueServer::Eune => PlatformRoute::EUN1,
            LeagueServer::Euw => PlatformRoute::EUW1,
            LeagueServer::Jp => PlatformRoute::JP1,
            LeagueServer::Kr => PlatformRoute::KR,
            LeagueServer::Lan => PlatformRoute::LA1,
            LeagueServer::Las => PlatformRoute::LA2,
            LeagueServer::Na => PlatformRoute::NA1,
            LeagueServer::Oce => PlatformRoute::OC1,
            LeagueServer::Ru => PlatformRoute::RU,
            LeagueServer::Tr => PlatformRoute::TR1,
            LeagueServer::Sg => PlatformRoute::SG2,
            LeagueServer::Ph => PlatformRoute::PH2,
            LeagueServer::Vn => PlatformRoute::VN2,
            LeagueServer::Tw => PlatformRoute::TW2,
            LeagueServer::Th => PlatformRoute::TH2,
            LeagueServer::Mena => PlatformRoute::ME1,
            LeagueServer::Pbe => PlatformRoute::PBE1,
        }
    }
}
