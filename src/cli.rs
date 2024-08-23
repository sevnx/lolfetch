use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    // Common options
    /// Your Riot ID (e.g. abc#1234)
    #[clap(long)]
    pub riot_id: String,

    /// Server the account is registered on
    #[clap(long)]
    pub server: LeagueServer,

    /// Image display option
    #[clap(long, default_value = "Rank")]
    pub display: ImageDisplay,

    /// Number of fetched matches
    #[clap(long, value_parser=parse_number_of_parsed_games)]
    pub matches: i32,

    /// Custom image link for the ASCII art
    #[clap(long, required_if_eq("display", "Custom"))]
    pub path: Option<String>,
}

/// Image display options
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
#[clap(rename_all = "PascalCase")]
pub enum ImageDisplay {
    /// Displays the rank
    #[default]
    Rank,

    /// Displays the champion icon with the highest mastery
    Mastery,

    /// Displays the icon used by the summoner
    Icon,

    /// Displays the champion icon with the highest mastery
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

fn parse_number_of_parsed_games(s: &str) -> Result<i32, String> {
    /// We limit the number of matches to 75, as Riot API has a rate limit of 100 requests / 2 min
    const MAX_MATCHES: i32 = 75;
    match s.parse() {
        Ok(n) if n <= 10 && n > 0 => Ok(n),
        Ok(_) => Err(format!(
            "Number of games must be between 1 and {MAX_MATCHES}"
        )),
        Err(_) => Err("Invalid number of games".to_string()),
    }
}
