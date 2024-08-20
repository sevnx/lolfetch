use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// Your Riot ID (e.g. abc#1234)
    #[clap(long)]
    pub riot_id: String,

    /// Server the account is registered on
    #[clap(long)]
    pub server: LeagueServer,

    /// Custom image link for the ASCII art
    #[clap(long)]
    pub path: Option<String>,
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
