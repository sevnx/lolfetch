use crate::cli::LeagueServer;
use riven::consts::PlatformRoute;

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
