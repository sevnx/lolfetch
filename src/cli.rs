//! This module handles the command line arguments for the application.

use crate::api::account::RiotId;
use anyhow::{Context, Error, Result};
use clap::{Command, Parser, Subcommand, ValueEnum};
use riven::consts::Champion;
use std::str::FromStr;

pub mod cache;
pub mod lolfetch;

/// Command line arguments for the application
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose mode
    #[clap(long)]
    pub verbose: bool,

    /// API key for the Riot API
    #[clap(long, default_value = "", value_parser = parse_api_key)]
    pub api_key: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Cache management
    Cache(cache::Cache),

    /// Default lolfetch mode
    Display(lolfetch::Lolfetch),
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

/// Parses the Riot API key from the command line
fn parse_api_key(key: &str) -> Result<String> {
    if key.is_empty() {
        std::env::var("RIOT_API_KEY").context("API key not found")
    } else {
        Ok(key.to_string())
    }
}
