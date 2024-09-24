//! This module regroups the configuration of the application.
//! It is used to load the configuration from CLI arguments and environment variables.

use crate::{
    api::account::RiotId,
    cache::CacheSaveOptions,
    cli::{
        self,
        lolfetch::{DisplayConfig, ImageSource, InfoKind},
    },
};
use anyhow::{Context, Result};
use riven::consts::{Champion, PlatformRoute};

/// Configuration of the application
#[derive(Debug, Clone)]
pub struct Globals {
    /// Information whether to save the cache or not
    pub cache_save: CacheSaveOptions,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// Account information
    pub account: Account,

    /// Type of image to display
    pub image: Image,

    /// Display mode
    pub mode: InfoKind,

    /// Information whether to save the cache or not
    pub globals: Globals,
}

impl Config {
    pub fn from_cli(value: cli::lolfetch::Lolfetch) -> Result<Self> {
        Ok(Self {
            account: Account {
                riot_id: value.summoner.riot_id,
                server: value.summoner.server.into(),
            },
            image: Self::parse_image_config(value.display_config)
                .context("Failed to parse image")?,
            mode: value.info_config,
            globals: Globals {
                cache_save: CacheSaveOptions::from_bool(!value.globals.no_save),
            },
        })
    }

    fn parse_image_config(display: DisplayConfig) -> Result<Image> {
        Ok(match display.image {
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
}

/// Summoner information
#[derive(Debug, Clone)]
pub struct Account {
    /// Riot ID of the summoner
    pub riot_id: RiotId,

    /// Server the account is registered on
    pub server: PlatformRoute,
}

impl From<cli::SummonerConfig> for Account {
    fn from(value: cli::SummonerConfig) -> Self {
        Self {
            riot_id: value.riot_id,
            server: value.server.into(),
        }
    }
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
