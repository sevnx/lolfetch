//! lolfetch CLI module

use super::SummonerConfig;
use crate::api::account::RiotId;
use anyhow::{Context, Error, Result};
use clap::{Command, Parser, Subcommand, ValueEnum};
use riven::consts::Champion;
use std::str::FromStr;

/// CLI arguments for the default lolfetch mode
#[derive(Parser, Debug)]
pub struct Lolfetch {
    /// Summoner information
    #[command(flatten)]
    pub summoner: SummonerConfig,

    /// Display options
    #[command(flatten)]
    pub display_config: DisplayConfig,

    /// Info display options
    #[command(flatten)]
    pub info_config: InfoOptions,
}

#[derive(Parser, Debug, Clone)]
pub struct DisplayConfig {
    /// Image source for the ASCII art
    #[clap(long, default_value = "Default")]
    pub display: ImageSource,

    /// Name of the champion icon to display
    #[clap(long, required_if_eq("display", "ChampionIcon"), value_parser = parse_champion)]
    pub champion: Option<Champion>,

    /// Link to the custom image to display
    #[clap(long, required_if_eq("display", "Custom"))]
    pub custom_img_url: Option<String>,
}

/// Image display options
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
#[clap(rename_all = "PascalCase")]
pub enum ImageSource {
    /// Default image display, based on the display type
    #[default]
    Default,

    /// Displays the rank of the player
    RankIcon,

    /// Displays the icon of a champion
    ChampionIcon,

    /// Displays the icon of the summoner
    SummonerIcon,

    /// Displays a custom image
    Custom,
}

/// Info display options
#[derive(Parser, Debug, Clone)]
pub struct InfoOptions {
    /// Type of information to display
    #[clap(long)]
    pub info: InfoType,

    /// Number of games to fetch for ranked statistics
    #[clap(long, default_value = "10", value_parser = parse_number_of_parsed_games)]
    pub games: i32,

    /// Number of top champions to display
    #[clap(long, default_value = "5")]
    pub top_champions: i32,

    /// Number of mastery champions to fetch
    #[clap(long, default_value = "10")]
    pub mastery_champions: i32,

    /// Number of recent matches to display
    #[clap(long, default_value = "5")]
    pub recent_matches: i32,
    // TODO: Define custom info options
}

/// The type of information to display in the application
#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "PascalCase")]
pub enum InfoType {
    /// Displays the information about ranked games
    Ranked,

    /// Displays the information about champion mastery
    Mastery,

    /// Displays the information about recent matches
    RecentMatches,

    /// Displays custom information
    Custom,
}

/// Parses the champion name from the command line
fn parse_champion(champion_name: &str) -> Result<Champion, Error> {
    Champion::from_str(champion_name).context("Invalid champion name")
}

/// Parses the number of games to fetch for ranked statistics
fn parse_number_of_parsed_games(s: &str) -> Result<i32, String> {
    /// We limit the max number of fetched game, as Riot API has a rate limit of 100 requests / 2 min
    const MAX_MATCHES: i32 = 90;
    match s.parse() {
        Ok(n) if n <= MAX_MATCHES && n > 0 => Ok(n),
        Ok(_) => Err(format!(
            "Number of games must be between 1 and {MAX_MATCHES}"
        )),
        Err(_) => Err("Invalid number of games".to_string()),
    }
}
