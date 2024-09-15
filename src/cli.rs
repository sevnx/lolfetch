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
    pub command: Option<Commands>,

    /// Verbose mode
    #[clap(long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Cache management
    Cache(cache::Cache),
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
