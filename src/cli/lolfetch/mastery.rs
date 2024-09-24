//! Mastery display options

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Mastery {
    /// Number of games to fetch for statistics
    #[clap(long, default_value = "10")]
    pub games: i32,

    /// Number of mastery champions to display
    #[clap(long, default_value = "10")]
    pub mastery_champions: i32,
}
