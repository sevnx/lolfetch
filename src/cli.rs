use std::str::FromStr;
use clap::{Parser, ValueEnum};
use riven::consts::Champion;
use crate::api::account::RiotId;
use anyhow::{Context, Error, Result};

/// Command line arguments for the application
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
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
pub struct SummonerConfig {
    /// Your Riot ID (e.g. abc#1234)
    #[clap(long, value_parser = RiotId::from_str)]
    pub riot_id: RiotId,

    /// Server the account is registered on
    #[clap(long)]
    pub server: LeagueServer,
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

    /// OPTIONAL: Number of matches to fetch for statistics (`Ranked`, `Mastery`, `RecentMatches`)
    #[clap(
        long, 
        default_value = "10", 
        value_parser = parse_number_of_parsed_games, 
        required_if_eq("info", "Ranked"), 
        required_if_eq("info", "Mastery"), 
    )]
    pub games: i32,

    /// OPTIONAL: Number of top champions to display
    #[clap(long, default_value = "5", required_if_eq("info", "Ranked"))]
    pub top_champions: i32,

    /// OPTIONAL: Number of mastery champions to fetch
    #[clap(long, default_value = "10", required_if_eq("info", "Mastery"))]
    pub mastery_champions: i32,

    /// OPTIONAL: Number of recent matches to display
    #[clap(long, default_value = "10", required_if_eq("info", "RecentMatches"))]
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

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "screaming_snake_case")]
pub enum LeagueServer {
    Na,
    Euw,
    Eune,
    Oce,
    Kr,
    Jp,
    Br,
    Las,
    Lan,
    Ru,
    Tr,
    Sg,
    Ph,
    Vn,
    Tw,
    Th,
    Mena,
    Pbe,
}

/// Parses the champion name from the command line
fn parse_champion(champion_name: &str) -> Result<Champion, Error> {
    Champion::from_str(champion_name).context("Invalid champion name")
}


/// Parses the number of games to fetch for ranked statistics
fn parse_number_of_parsed_games(s: &str) -> Result<i32, String> {
    /// We limit the max number of matches to 75, as Riot API has a rate limit of 100 requests / 2 min
    const MAX_MATCHES: i32 = 75;
    match s.parse() {
        Ok(n) if n <= 10 && n > 0 => Ok(n),
        Ok(_) => Err(format!(
            "Number of games must be between 1 and {MAX_MATCHES}"
        )),
        Err(_) => Err("Invalid number of games".to_string()),
    }
}
