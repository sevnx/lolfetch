//! Cache CLI module.

use super::SummonerConfig;
use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cache {
    #[command(subcommand)]
    pub action: CacheAction,
}

#[derive(Subcommand, Debug)]
pub enum CacheAction {
    Clear(Clear),
    Load(Load),
}

/// CLI clear cache arguments
#[derive(Parser, Debug)]
pub struct Clear {
    /// Summoner information
    #[command(flatten)]
    pub summoner: Option<SummonerConfig>,
}

/// CLI load cache arguments
#[derive(Parser, Debug)]
pub struct Load {
    /// Summoner information
    #[command(flatten)]
    pub summoner: SummonerConfig,

    /// Number of matches to load
    #[clap(long)]
    pub matches: Option<i32>,
}
