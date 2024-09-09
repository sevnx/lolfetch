//! Caching methods for fetched data.

use crate::{
    api, config,
    models::matches::{MatchInfo, MatchMap},
};
use anyhow::Result;
use riven::{
    consts::PlatformRoute,
    models::{lol_status_v4::PlatformData, match_v5, summoner_v4::Summoner},
};
use std::{collections::HashMap, fs, io, path::PathBuf};

/// Returns the cache directory for lolfetch.
fn get_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Cache directory not found",
        ))?
        .join(PathBuf::from("lolfetch"));
    if !cache_dir.exists() {
        std::fs::create_dir(&cache_dir)?;
    }

    Ok(cache_dir)
}

/// Returns the cache directory for a summoner.
fn get_summoner_cache_dir(summoner: &Summoner, route: PlatformRoute) -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let summonner_cache_dir = cache_dir
        .join(PathBuf::from("summoner"))
        .join(PathBuf::from(route.to_string()))
        .join(PathBuf::from(&summoner.puuid));

    if !summonner_cache_dir.exists() {
        std::fs::create_dir_all(&summonner_cache_dir)?;
    }

    Ok(summonner_cache_dir)
}

pub type MatchId = i64;

pub struct Cache<'a> {
    summoner: &'a Summoner,
    route: PlatformRoute,
    match_info: MatchMap,
}

impl<'a> Cache<'a> {
    pub fn load_cache(summoner: &'a Summoner, route: PlatformRoute) -> anyhow::Result<Self> {
        let cache_dir = get_summoner_cache_dir(summoner, route)?;
        let file_path = cache_dir.join("matches.json");

        if !file_path.exists() {
            return Ok(Self {
                match_info: HashMap::new(),
                route,
                summoner,
            });
        }

        let cache_str = std::fs::read_to_string(&file_path)?;
        fs::remove_file(&file_path).map_err(|e| format!("Failed to remove file : {:?}", e));
        let cache_map = serde_json::from_str::<HashMap<MatchId, MatchInfo>>(&cache_str)?;

        Ok(Self {
            summoner,
            route,
            match_info: cache_map,
        })
    }

    pub fn insert(&mut self, match_id: MatchId, info: MatchInfo) {
        self.match_info.insert(match_id, info);
    }

    pub fn is_empty(&self) -> bool {
        self.match_info.is_empty()
    }

    /// Saves the cache to storage, and returns its content.
    pub fn save(self) -> anyhow::Result<HashMap<MatchId, MatchInfo>> {
        let serialized = serde_json::to_string(&self.match_info)?;

        let file_path = get_summoner_cache_dir(self.summoner, self.route)?.join("matches.json");
        fs::write(&file_path, serialized)?;

        Ok(self.match_info)
    }
}
