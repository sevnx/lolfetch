use crate::cli::LeagueServer;
use riven::consts::{PlatformRoute, RegionalRoute};

/// Trait that converts a server to a regional route.
pub trait RegionalRouteFromServer {
    fn from(&self) -> RegionalRoute;
}

impl RegionalRouteFromServer for PlatformRoute {
    fn from(&self) -> RegionalRoute {
        use PlatformRoute::{
            BR1, EUN1, EUW1, JP1, KR, LA1, LA2, ME1, NA1, OC1, PBE1, PH2, RU, SG2, TH2, TR1, TW2,
            VN2,
        };
        match self {
            PBE1 | BR1 | LA1 | LA2 | NA1 => RegionalRoute::AMERICAS,
            EUN1 | EUW1 | RU | TR1 | ME1 => RegionalRoute::EUROPE,
            JP1 | KR | OC1 | SG2 | PH2 | VN2 | TW2 | TH2 => RegionalRoute::ASIA,
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
