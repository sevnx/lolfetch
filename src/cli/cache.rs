//! Cache CLI module.

use clap::{command, Parser, Subcommand};
use riven::consts::QueueType;

use super::SummonerConfig;

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

    /// Queue to load
    #[clap(long, value_parser = parse_queue_type, default_value = "solo")]
    pub queue: QueueType,

    /// Number of matches to load
    #[clap(long)]
    pub matches: Option<i32>,
}

/// Parser for `QueueType`
fn parse_queue_type(s: &str) -> Result<QueueType, String> {
    match s {
        "solo" => Ok(QueueType::RANKED_SOLO_5x5),
        "flex" => Ok(QueueType::RANKED_FLEX_SR),
        _ => Err("Invalid queue type".to_string()),
    }
}
