//! Ranked display options

use super::parse_number_of_parsed_games;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Ranked {
    /// Number of games to fetch for ranked statistics
    #[clap(long, default_value = "10", value_parser = parse_number_of_parsed_games)]
    pub games: i32,

    /// Number of top champions to display
    #[clap(long, default_value = "5")]
    pub top_champions: i32,

    /// Number of recent matches to display
    #[clap(long, default_value = "5")]
    pub recent_matches: i32,
}
