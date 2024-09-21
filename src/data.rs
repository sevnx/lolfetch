//! This module handles gathering information to display to the user.

use crate::{
    api::Data as ApiData,
    config::{Config, Mode},
    display::{DisplayableSectionKind, IMAGE_HEIGHT, IMAGE_WIDTH},
};
use champion_stats::RecentChampionInfo;
use lolfetch_ascii::ColoredArt;
use mastery::Mastery;
use match_history::MatchHistory;
use riven::models::match_v5;
use summoner::Summoner;
use thiserror::Error;

pub mod champion_stats;
pub mod mastery;
pub mod match_history;
pub mod summoner;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Failed to fetch image")]
    ImageFetchError(#[from] lolfetch_ascii::ArtProcessingError),

    #[error("Failed to process data")]
    IncorrectData(String),
}

pub struct ApplicationData {
    pub image: ColoredArt,
    pub sections: Vec<DisplayableSectionKind>,
}

impl ApplicationData {
    pub async fn process(data: ApiData, config: &Config) -> Result<Self, ProcessingError> {
        info!("Processing fetched data");

        let mut sections = Vec::new();
        match &config.mode {
            Mode::Ranked(ranked) => {
                // Name + Ranked champion stats + Recent Matches
                let ranked_summoner = Summoner::new(&config.account.riot_id, data.ranked);

                let matches: Vec<match_v5::Info> = match data.matches {
                    Some(matches) => matches.into_iter().map(|m| m.info).collect(),
                    None => return Err(ProcessingError::IncorrectData("No matches found".into())),
                };

                let champions =
                    RecentChampionInfo::new(&matches, &data.summoner, ranked.top_champions);

                let match_history =
                    MatchHistory::new(&matches, &data.summoner, ranked.recent_matches);

                sections.push(DisplayableSectionKind::Summoner(ranked_summoner));
                sections.push(DisplayableSectionKind::MatchHistory(match_history));
                sections.push(DisplayableSectionKind::RecentChampionInfo(champions));
            }
            Mode::Mastery(mastery) => {
                // Name + Masteries

                let summoner = Summoner::new(&config.account.riot_id, None);
                let mastery = Mastery::new(data.masteries, mastery.mastery_champions);

                sections.push(DisplayableSectionKind::Summoner(summoner));
                sections.push(DisplayableSectionKind::Mastery(mastery));
            }
            Mode::RecentMatches(recent) => {
                // Name + Recent Matches
                let ranked_summoner = Summoner::new(&config.account.riot_id, data.ranked);

                let matches: Vec<match_v5::Info> = match data.matches {
                    Some(matches) => matches.into_iter().map(|m| m.info).collect(),
                    None => return Err(ProcessingError::IncorrectData("No matches found".into())),
                };

                let match_history =
                    MatchHistory::new(&matches, &data.summoner, recent.recent_matches);

                sections.push(DisplayableSectionKind::Summoner(ranked_summoner));
                sections.push(DisplayableSectionKind::MatchHistory(match_history));
            }
            _ => {}
        }

        info!("Finished processing data");

        let image = lolfetch_ascii::from_url(&data.image_url, IMAGE_WIDTH, IMAGE_HEIGHT).await?;

        Ok(Self { image, sections })
    }
}
