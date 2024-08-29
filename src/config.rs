//! This module regroups the configuration of the application.
//! It is used to load the configuration from CLI arguments and environment variables.

use crate::api::account::RiotId;
use crate::cli::{Cli, DisplayConfig, ImageSource, InfoOptions, InfoType};
use anyhow::{Context, Result};
use riven::consts::{Champion, PlatformRoute};

#[derive(Debug, Clone)]
pub struct Config {
    /// API key for the Riot API
    pub api_key: String,

    /// Account information
    pub account: Account,

    /// Type of image to display
    pub image: Image,

    /// Display mode
    pub mode: Mode,
}

impl Config {
    pub fn from_cli(value: Cli) -> Result<Self> {
        Ok(Self {
            api_key: value.api_key,
            account: Account {
                riot_id: value.summoner.riot_id,
                server: value.summoner.server.into(),
            },
            image: Self::parse_image_config(value.display_config)?,
            mode: Self::parse_mode_config(&value.info_config),
        })
    }

    fn parse_image_config(display: DisplayConfig) -> Result<Image> {
        Ok(match display.display {
            ImageSource::Default => Image::Default,
            ImageSource::RankIcon => Image::RankIcon,
            ImageSource::ChampionIcon => {
                Image::ChampionIcon(display.champion.context("Champion icon not provided")?)
            }
            ImageSource::SummonerIcon => Image::SummonerIcon,
            ImageSource::Custom => Image::Custom(
                display
                    .custom_img_url
                    .context("Custom image URL not provided")?,
            ),
        })
    }

    fn parse_mode_config(info: &InfoOptions) -> Mode {
        match info.info {
            InfoType::Ranked => Mode::Ranked(Ranked {
                games: info.games,
                top_champions: info.top_champions,
                recent_matches: info.recent_matches,
            }),
            InfoType::Mastery => Mode::Mastery(Mastery {
                games: info.games,
                mastery_champions: info.mastery_champions,
            }),
            InfoType::RecentMatches => Mode::RecentMatches(RecentMatches {
                recent_matches: info.recent_matches,
            }),
            InfoType::Custom => Mode::Custom(Custom {}),
        }
    }
}

/// Summoner information
#[derive(Debug, Clone)]
pub struct Account {
    /// Riot ID of the summoner
    pub riot_id: RiotId,

    /// Server the account is registered on
    pub server: PlatformRoute,
}

#[derive(Debug, Clone)]
pub enum Image {
    /// Default image display, based on the display type
    Default,

    /// Displays the rank icon
    RankIcon,

    /// Displays the champion icon of a specifically chosen champion
    ChampionIcon(Champion),

    /// Displays the summoner icon
    SummonerIcon,

    /// Displays a custom image
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// Displays information aobut ranked games
    Ranked(Ranked),

    /// Displays information about champion mastery
    Mastery(Mastery),

    /// Displays information about recent matches
    RecentMatches(RecentMatches),

    /// Displays custom information
    Custom(Custom),
}

/// Configuration for the display of ranked information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ranked {
    /// Number of games to fetch
    pub games: i32,

    /// Top champions to display
    pub top_champions: i32,

    /// Number of recent matches to display
    pub recent_matches: i32,
}

/// Configuration for the display of mastery information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mastery {
    /// Number of games to fetch
    pub games: i32,

    /// Number of mastery champions to display
    pub mastery_champions: i32,
}

/// Configuration for the display of recent matches
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecentMatches {
    /// Number of games to fetch and display
    pub recent_matches: i32,
}

/// Configuration for the display of custom information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Custom {
    // TODO: Define the custom configuration
}
