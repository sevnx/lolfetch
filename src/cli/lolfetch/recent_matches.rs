//! Recent matches display options

use crate::cli::SummonerConfig;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct RecentMatches {
    /// Summoner information
    #[command(flatten)]
    pub summoner: SummonerConfig,

    /// Number of games to fetch and display
    #[clap(long, default_value = "5")]
    pub recent_matches: i32,
}
