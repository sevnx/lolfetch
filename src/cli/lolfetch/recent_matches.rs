//! Recent matches display options

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct RecentMatches {
    /// Number of games to fetch and display
    #[clap(long, default_value = "5")]
    pub recent_matches: i32,
}
