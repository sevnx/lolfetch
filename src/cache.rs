//! Caching methods for fetched data.

use crate::models::matches::{MatchInfo, MatchMap};
use anyhow::Context;
use riven::{consts::PlatformRoute, models::summoner_v4::Summoner};
use std::{collections::HashMap, fs, io, path::PathBuf};

/// Returns the cache directory for lolfetch.
fn get_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Cache directory not found",
        ))?
        .join(PathBuf::from("lolfetch"));

    Ok(cache_dir)
}

/// Returns the cache directory for a summoner.
fn get_summoner_cache_dir(summoner: &Summoner, route: PlatformRoute) -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let summonner_cache_dir = cache_dir
        .join(PathBuf::from("summoner"))
        .join(PathBuf::from(route.to_string()))
        .join(PathBuf::from(&summoner.puuid));

    Ok(summonner_cache_dir)
}

pub type MatchId = String;

pub struct Cache {
    summoner: Summoner,
    route: PlatformRoute,
    match_info: MatchMap,
}

impl Cache {
    fn new(summoner: Summoner, route: PlatformRoute) -> Self {
        Self {
            summoner,
            route,
            match_info: HashMap::new(),
        }
    }

    pub fn load_cache(summoner: Summoner, route: PlatformRoute) -> anyhow::Result<Self> {
        info!("Loading cache for summoner");

        let cache_dir = get_summoner_cache_dir(&summoner, route)?;

        if !cache_dir.exists() {
            return Ok(Self::new(summoner, route));
        }

        let file_path = cache_dir.join("matches.json");

        if !file_path.exists() {
            return Ok(Self::new(summoner, route));
        }

        let cache_str = std::fs::read_to_string(&file_path)?;
        fs::remove_file(&file_path)?;
        Ok(
            match serde_json::from_str::<HashMap<MatchId, MatchInfo>>(&cache_str) {
                Ok(cache) => Self {
                    summoner,
                    route,
                    match_info: cache,
                },
                Err(_) => Self::new(summoner, route),
            },
        )
    }

    pub fn insert(&mut self, match_id: MatchId, info: MatchInfo) {
        self.match_info.insert(match_id, info);
    }

    pub fn contains(&self, match_id: &MatchId) -> bool {
        self.match_info.contains_key(match_id)
    }

    pub fn is_empty(&self) -> bool {
        self.match_info.is_empty()
    }

    /// Saves the cache to storage, and returns its content.
    pub fn save(self) -> anyhow::Result<Vec<MatchInfo>> {
        let serialized = serde_json::to_string(&self.match_info)?;

        let dir = get_summoner_cache_dir(&self.summoner, self.route)?;

        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file_path = dir.join("matches.json");
        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }
        fs::write(&file_path, serialized)?;

        let mut match_vec: Vec<MatchInfo> = self.match_info.into_values().collect();

        // Reversed sort
        match_vec.sort_by(|a, b| b.info.game_creation.cmp(&a.info.game_creation));

        Ok(match_vec)
    }

    /// Clears the cache
    pub fn clear(summoner: Option<(Summoner, PlatformRoute)>) -> anyhow::Result<()> {
        match summoner {
            Some(summoner) => {
                let dir = get_summoner_cache_dir(&summoner.0, summoner.1)?;
                if dir.exists() {
                    info!("Clearing cache for summoner");
                    fs::remove_dir_all(dir).context("Failed to clear cache")
                } else {
                    warn!("Cache directory does not exist for summoner");
                    Ok(())
                }
            }
            None => {
                info!("Clearing cache for all summoners");
                let dir = get_cache_dir()?;
                if dir.exists() {
                    info!("Clearing cache");
                    fs::remove_dir_all(dir).context("Failed to clear cache")
                } else {
                    warn!("Cache directory does not exist");
                    Ok(())
                }
            }
        }
    }
}
