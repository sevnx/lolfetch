//! lolfetch CLI module

use super::SummonerConfig;
use anyhow::{Context, Error, Result};
use clap::{Parser, ValueEnum};
use riven::consts::Champion;
use std::str::FromStr;

// Crate modules
pub mod custom;
pub mod mastery;
pub mod ranked;
pub mod recent_matches;

/// CLI arguments for the default lolfetch mode (display)
#[derive(Parser, Debug)]
pub struct Lolfetch {
    /// Info display options
    #[command(subcommand)]
    pub info_config: InfoKind,

    /// Summoner information
    #[command(flatten)]
    pub summoner: SummonerConfig,

    /// Display options
    #[command(flatten)]
    pub display_config: DisplayConfig,

    /// Global flags
    #[command(flatten)]
    pub globals: Globals,
}

/// Global flag configuration
#[derive(Parser, Debug, Clone)]
pub struct Globals {
    /// Don't save the cache to disk
    #[clap(long)]
    pub no_save: bool,
}

/// Configuration for the image that is displayed
#[derive(Parser, Debug, Clone)]
pub struct DisplayConfig {
    /// Image source for the ASCII art
    #[clap(long, default_value = "Default")]
    pub image: ImageSource,

    /// Name of the champion icon to display
    #[clap(long, required_if_eq("image", "ChampionIcon"), value_parser = parse_champion)]
    pub champion: Option<Champion>,

    /// Link to the custom image to display
    #[clap(long, required_if_eq("image", "Custom"))]
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
    #[clap(name = "rank")]
    RankIcon,

    #[clap(name = "champion")]
    /// Displays the icon of a champion
    ChampionIcon,

    #[clap(name = "profile")]
    /// Displays the icon of the summoner
    SummonerIcon,

    /// Displays a custom image
    Custom,
}

#[derive(clap::Subcommand, Debug, Clone)]
// TODO: Add lolfetch
pub enum InfoKind {
    /// Ranked information
    Ranked(ranked::Ranked),

    /// Mastery information
    Mastery(mastery::Mastery),

    /// Recent matches information
    RecentMatches(recent_matches::RecentMatches),

    /// Custom information
    Custom(custom::Custom),
}

/// Parses the champion name from the command line
fn parse_champion(champion_name: &str) -> Result<Champion, Error> {
    Champion::from_str(champion_name).context("Invalid champion name")
}

/// Parses the number of games to fetch for ranked statistics
fn parse_number_of_parsed_games(s: &str) -> Result<i32, String> {
    match s.parse() {
        Ok(games) => {
            if games > 0 {
                Ok(games)
            } else {
                Err("Number of games must be greater than 0".to_string())
            }
        }
        Err(_) => Err("Invalid number of games".to_string()),
    }
}
