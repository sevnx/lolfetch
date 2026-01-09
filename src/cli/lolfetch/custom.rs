//! Custom display configuration

use crate::cli::SummonerConfig;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Custom {
    /// Summoner information
    #[command(flatten)]
    pub summoner: SummonerConfig,
}
