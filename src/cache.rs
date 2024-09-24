//! Caching methods for fetched data.

use crate::models::matches::{MatchInfo, MatchMap};
use anyhow::Context;
use riven::{consts::PlatformRoute, models::summoner_v4::Summoner};
use std::{
    collections::HashMap,
    fs::{self, create_dir_all, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

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
    match_info: MatchMap,
    cache_file_lock: fs::File,
}

#[derive(Debug)]
pub enum CacheInsertError {
    AlreadyExists,
    Remake,
    PatchMismatch,
}

impl Cache {
    fn new(cache_file_lock: fs::File) -> Self {
        Self {
            match_info: HashMap::new(),
            cache_file_lock,
        }
    }

    pub fn load_cache_from_file(summoner: Summoner, route: PlatformRoute) -> anyhow::Result<Self> {
        info!("Loading cache for summoner");

        let cache_dir = get_summoner_cache_dir(&summoner, route)?;
        if !cache_dir.exists() {
            create_dir_all(&cache_dir)?;
        }

        let file_path = cache_dir.join("matches.json");

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&file_path)?;

        if file.metadata()?.len() == 0 {
            return Ok(Self::new(file));
        }

        let mut cache_str = String::new();
        file.read_to_string(&mut cache_str)?;

        let cache = match serde_json::from_str::<HashMap<MatchId, MatchInfo>>(&cache_str) {
            Ok(cache) => cache,
            Err(err) => {
                warn!("Failed to deserialize cache: {:?}", err);
                HashMap::new()
            }
        };

        Ok(Self {
            match_info: cache,
            cache_file_lock: file,
        })
    }

    pub async fn insert(
        &mut self,
        match_id: MatchId,
        info: MatchInfo,
    ) -> Result<(), CacheInsertError> {
        if self.match_info.contains_key(&match_id) {
            return Err(CacheInsertError::AlreadyExists);
        }

        if info.is_remake() {
            return Err(CacheInsertError::Remake);
        }

        if !info.is_current_split().await {
            return Err(CacheInsertError::PatchMismatch);
        }

        self.match_info.insert(match_id, info);
        Ok(())
    }

    pub fn contains(&self, match_id: &MatchId) -> bool {
        self.match_info.contains_key(match_id)
    }

    pub fn is_empty(&self) -> bool {
        self.match_info.is_empty()
    }

    pub fn len(&self) -> usize {
        self.match_info.len()
    }

    /// Saves the cache to storage, and returns its content.
    pub fn save_to_file(mut self) -> anyhow::Result<Vec<MatchInfo>> {
        let serialized =
            serde_json::to_string(&self.match_info).context("Failed to serialize cache")?;

        // Clear the file
        self.cache_file_lock
            .set_len(0)
            .context("Failed to clear cache file")?;
        self.cache_file_lock
            .seek(SeekFrom::Start(0))
            .context("Failed to seek to start of cache file")?;

        // Write the new cache
        self.cache_file_lock
            .write_all(serialized.as_bytes())
            .context("Failed to write cache file")?;
        self.cache_file_lock
            .flush()
            .context("Failed to flush cache file")?;
        self.cache_file_lock
            .sync_all()
            .context("Failed to sync cache file")?;
        drop(self.cache_file_lock);

        info!("Saved cache to file");

        let mut match_vec: Vec<MatchInfo> = self.match_info.into_values().collect();

        // Reversed sort
        match_vec.sort_by(|a, b| b.info.game_creation.cmp(&a.info.game_creation));

        Ok(match_vec)
    }

    /// Clears the cache
    pub fn clear(summoner: Option<(Summoner, PlatformRoute)>) -> anyhow::Result<()> {
        if let Some(summoner) = summoner {
            let dir = get_summoner_cache_dir(&summoner.0, summoner.1)?;
            if dir.exists() {
                info!("Clearing cache for summoner");
                fs::remove_dir_all(dir).context("Failed to clear cache")
            } else {
                warn!("Cache directory does not exist for summoner");
                Ok(())
            }
        } else {
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
