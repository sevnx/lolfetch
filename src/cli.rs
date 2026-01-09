//! This module handles the command line arguments for the application.

use crate::api::account::RiotId;
use clap::{Parser, Subcommand, ValueEnum};

pub mod cache;
pub mod lolfetch;

/// Command line arguments for the application
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose mode
    #[clap(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Cache management
    Cache(cache::Cache),

    /// Display information about a League of Legends account
    Display(lolfetch::Lolfetch),
}

#[derive(Parser, Debug, Clone)]
pub struct SummonerConfig {
    /// Your Riot ID (e.g. abc#1234)
    #[clap(value_parser = RiotId::from_str)]
    pub riot_id: RiotId,

    /// Server the account is registered on
    #[clap()]
    pub server: LeagueServer,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
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
    Vn,
    Tw,
    Mena,
    Pbe,
}
