use riven::{
    consts::Champion,
    models::{
        match_v5::{self, Participant, Team},
        summoner_v4::{self, Summoner},
    },
};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MatchPlayerInfoError {
    #[error("{0}")]
    ParticipantNotFound(#[from] ParticipantGetterError),

    #[error("{0}")]
    TeamNotFound(#[from] TeamGetterError),

    #[error("{0}")]
    MaxTimeNotFound(#[from] GameTimeGetterError),

    #[error("{0}")]
    InvalidPosition(#[from] TeamPositionError),

    #[error("Champion not found")]
    ChampionNotFound,
}

pub struct MatchPlayerInfo {
    /// Champion played in the match.
    pub champion: Champion,
    /// KDA
    pub kda: Kda,
    /// Minions killed in the match.
    pub minions_killed: i32,
    /// Time played in the match.
    pub time_played: i32,
    /// Whether the player won the match.
    pub game_result: GameResult,
    /// Position in the team.
    pub team_position: TeamPosition,
}

impl MatchPlayerInfo {
    pub fn from_match(
        match_data: &match_v5::Match,
        summoner: &summoner_v4::Summoner,
    ) -> Result<Self, MatchPlayerInfoError> {
        let participant = match_data.get_participant(summoner)?;
        let team = match_data.get_my_team(participant)?;
        let max_time = match_data.get_max_time()?;
        let game_result = match team.win {
            true => GameResult::Win,
            false => GameResult::Loss,
        };

        Ok(Self {
            champion: participant
                .champion()
                .map_err(|_| MatchPlayerInfoError::ChampionNotFound)?,
            kda: Kda(participant.kills, participant.deaths, participant.assists),
            minions_killed: participant.total_minions_killed
                + participant.total_ally_jungle_minions_killed.unwrap_or(0)
                + participant.total_enemy_jungle_minions_killed.unwrap_or(0),
            time_played: max_time,
            game_result,
            team_position: participant.team_position.clone().try_into()?,
        })
    }
}

#[derive(Debug)]
pub enum TeamPosition {
    Top,
    Jungle,
    Mid,
    Bot,
    Support,
}

#[derive(Debug, Error)]
pub enum TeamPositionError {
    #[error("Invalid position")]
    InvalidPosition,
}

impl TryFrom<String> for TeamPosition {
    type Error = TeamPositionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "TOP" => Ok(Self::Top),
            "JUNGLE" => Ok(Self::Jungle),
            "MIDDLE" => Ok(Self::Mid),
            "BOTTOM" => Ok(Self::Bot),
            "UTILITY" => Ok(Self::Support),
            _ => Err(TeamPositionError::InvalidPosition),
        }
    }
}

impl fmt::Display for TeamPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "TOP"),
            Self::Jungle => write!(f, "JGL"),
            Self::Mid => write!(f, "MID"),
            Self::Bot => write!(f, "BOT"),
            Self::Support => write!(f, "SUP"),
        }
    }
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Loss,
}

#[derive(Debug, Error)]
pub enum ParticipantGetterError {
    #[error("Failed to find participant")]
    ParticipantNotFound,
}

pub trait ParticipantGetter {
    fn get_participant(&self, summoner: &Summoner) -> Result<&Participant, ParticipantGetterError>;
}

impl ParticipantGetter for match_v5::Match {
    fn get_participant(&self, summoner: &Summoner) -> Result<&Participant, ParticipantGetterError> {
        self.info
            .participants
            .iter()
            .find(|p| p.puuid == summoner.puuid)
            .ok_or(ParticipantGetterError::ParticipantNotFound)
    }
}

#[derive(Debug, Error)]
pub enum TeamGetterError {
    #[error("Failed to find team")]
    TeamNotFound,
}

pub trait TeamGetter {
    fn get_my_team(&self, participant: &Participant) -> Result<&Team, TeamGetterError>;
}

impl TeamGetter for match_v5::Match {
    fn get_my_team(&self, participant: &Participant) -> Result<&Team, TeamGetterError> {
        self.info
            .teams
            .iter()
            .find(|t| t.team_id == participant.team_id)
            .ok_or(TeamGetterError::TeamNotFound)
    }
}

#[derive(Debug, Error)]
pub enum GameTimeGetterError {
    #[error("Failed to find max time")]
    MaxTimeNotFound,
}

pub trait GameTimeGetter {
    fn get_max_time(&self) -> Result<i32, GameTimeGetterError>;
}

impl GameTimeGetter for match_v5::Match {
    fn get_max_time(&self) -> Result<i32, GameTimeGetterError> {
        self.info
            .participants
            .iter()
            .map(|p| p.time_played)
            .max()
            .ok_or(GameTimeGetterError::MaxTimeNotFound)
    }
}

pub struct Kda(pub i32, pub i32, pub i32);

impl Kda {
    pub fn get_kda(&self) -> Option<f64> {
        let Self(kills, deaths, assists) = self;
        if *deaths == 0 {
            None
        } else {
            Some(f64::from(kills + assists) / f64::from(*deaths))
        }
    }

    pub fn add(&mut self, other: &Self) {
        let Self(kills, deaths, assists) = self;
        let Self(other_kills, other_deaths, other_assists) = other;
        *kills += other_kills;
        *deaths += other_deaths;
        *assists += other_assists;
    }
}

impl fmt::Display for Kda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(kills, deaths, assists) = self;
        write!(f, "{kills}/{deaths}/{assists}")
    }
}
