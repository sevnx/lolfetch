use riven::{
    consts::Champion,
    models::{
        match_v5::{self, Participant, Team},
        summoner_v4::{self, Summoner},
    },
};
use std::{collections::HashMap, fmt};
use thiserror::Error;

use crate::api::tooling::{ranked_schedule::get_split_from_patch, static_data::get_latest_patch};

pub type MatchId = String;
pub type MatchMap = HashMap<MatchId, MatchInfo>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Match information.
pub struct MatchInfo {
    pub id: MatchId,
    pub info: match_v5::Info,
    pub timeline: Option<match_v5::InfoTimeLine>,
}

impl MatchInfo {
    pub fn is_remake(&self) -> bool {
        const MINUTES_UNTIL_REMAKE: i64 = 3;
        self.info.game_duration < MINUTES_UNTIL_REMAKE * 60
    }

    pub async fn is_current_split(&self) -> bool {
        get_split_from_patch(&self.info.game_version).expect("Failed to get split from patch")
            == get_split_from_patch(get_latest_patch().await)
                .expect("Failed to get split from patch")
    }
}

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
    /// GD@15
    pub gold_diff_15: Option<i32>,
}

impl MatchPlayerInfo {
    pub fn from_match_info(
        match_data: &MatchInfo,
        summoner: &summoner_v4::Summoner,
    ) -> Result<Self, MatchPlayerInfoError> {
        let participant = match_data.info.get_participant(summoner)?;
        let team = match_data.info.get_my_team(participant)?;
        let max_time = match_data.info.get_max_time()?;
        let gold_diff = match_data.get_gold_diff(summoner, 60 * 15);
        let game_result = if team.win {
            GameResult::Win
        } else {
            GameResult::Loss
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
            gold_diff_15: gold_diff,
        })
    }
}

pub trait GoldDiffGetter {
    fn get_gold_diff(&self, summoner: &Summoner, seconds: i32) -> Option<i32>;
}

impl GoldDiffGetter for MatchInfo {
    fn get_gold_diff(&self, summoner: &Summoner, seconds: i32) -> Option<i32> {
        let participant = self.info.get_participant(summoner).ok()?;
        info!("{:?}", participant.team_position);
        let position = TeamPosition::try_from(participant.team_position.clone()).ok()?;

        let enemy_laner = self.info.participants.iter().find(|p| {
            p.team_position == position.to_riot_api_string() && p.team_id != participant.team_id
        })?;

        let frame = self.timeline.as_ref()?.get_frame(seconds)?;

        let frame = frame.participant_frames.clone().unwrap();

        let enemy = frame.get(&enemy_laner.participant_id)?;
        let me = frame.get(&participant.participant_id)?;

        Some(me.total_gold - enemy.total_gold)
    }
}

pub trait FrameGetter {
    fn get_frame(&self, seconds: i32) -> Option<&match_v5::FramesTimeLine>;
}

impl FrameGetter for match_v5::InfoTimeLine {
    fn get_frame(&self, seconds: i32) -> Option<&match_v5::FramesTimeLine> {
        let sec_timestamp = seconds * 1000;
        self.frames
            .iter()
            .find(|&frame| frame.timestamp > sec_timestamp)
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

impl TeamPosition {
    pub fn to_riot_api_string(&self) -> &'static str {
        match self {
            Self::Top => "TOP",
            Self::Jungle => "JUNGLE",
            Self::Mid => "MIDDLE",
            Self::Bot => "BOTTOM",
            Self::Support => "UTILITY",
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

impl ParticipantGetter for match_v5::Info {
    fn get_participant(&self, summoner: &Summoner) -> Result<&Participant, ParticipantGetterError> {
        self.participants
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

impl TeamGetter for match_v5::Info {
    fn get_my_team(&self, participant: &Participant) -> Result<&Team, TeamGetterError> {
        self.teams
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

impl GameTimeGetter for match_v5::Info {
    fn get_max_time(&self) -> Result<i32, GameTimeGetterError> {
        self.participants
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
