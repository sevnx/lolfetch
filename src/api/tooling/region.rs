use crate::cli::LeagueServer;
use riven::consts::PlatformRoute;

impl From<LeagueServer> for PlatformRoute {
    fn from(server: LeagueServer) -> Self {
        match server {
            LeagueServer::Br => Self::BR1,
            LeagueServer::Eune => Self::EUN1,
            LeagueServer::Euw => Self::EUW1,
            LeagueServer::Jp => Self::JP1,
            LeagueServer::Kr => Self::KR,
            LeagueServer::Lan => Self::LA1,
            LeagueServer::Las => Self::LA2,
            LeagueServer::Na => Self::NA1,
            LeagueServer::Oce => Self::OC1,
            LeagueServer::Ru => Self::RU,
            LeagueServer::Tr => Self::TR1,
            LeagueServer::Sg => Self::SG2,
            LeagueServer::Ph => Self::PH2,
            LeagueServer::Vn => Self::VN2,
            LeagueServer::Tw => Self::TW2,
            LeagueServer::Th => Self::TH2,
            LeagueServer::Mena => Self::ME1,
            LeagueServer::Pbe => Self::PBE1,
        }
    }
}
