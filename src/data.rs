//! This module handles gathering information to display to the user.

use champion_stats::RecentChampionInfo;
use lolfetch_ascii::ColoredArt;
use mastery::Mastery;
use match_history::MatchHistory;
use summoner::Summoner;

use crate::{
    api::Data as ApiData,
    config::{Config, Mode},
    display::{DisplayableSectionKind, IMAGE_HEIGHT, IMAGE_WIDTH},
};

pub mod champion_stats;
pub mod mastery;
pub mod match_history;
pub mod summoner;

pub struct ApplicationData {
    pub image: ColoredArt,
    pub sections: Vec<DisplayableSectionKind>,
}

impl ApplicationData {
    pub async fn process(data: ApiData, config: &Config) -> Self {
        let mut sections = Vec::new();
        match &config.mode {
            Mode::Ranked(ranked) => {
                // Name + Ranked champion stats + Recent Matches
                let ranked_summoner = Summoner::new(&config.account.riot_id, data.ranked);

                let matches = data.matches.unwrap();

                let champions =
                    RecentChampionInfo::new(&matches, &data.summoner, ranked.top_champions)
                        .unwrap();
                let match_history =
                    MatchHistory::new(&matches, &data.summoner, ranked.recent_matches).unwrap();

                sections.push(DisplayableSectionKind::Summoner(ranked_summoner));
                sections.push(DisplayableSectionKind::MatchHistory(match_history));
                sections.push(DisplayableSectionKind::RecentChampionInfo(champions));
            }
            Mode::Mastery(mastery) => {
                // Name + Masteries

                let summoner = Summoner::new(&config.account.riot_id, None);
                let mastery = Mastery::new(data.masteries.unwrap(), mastery.mastery_champions);

                sections.push(DisplayableSectionKind::Summoner(summoner));
                sections.push(DisplayableSectionKind::Mastery(mastery));
            }
            Mode::RecentMatches(recent) => {
                // Name + Recent Matches
                let ranked_summoner = Summoner::new(&config.account.riot_id, data.ranked);
                let match_history = MatchHistory::new(
                    &data.matches.unwrap(),
                    &data.summoner,
                    recent.recent_matches,
                )
                .unwrap();

                sections.push(DisplayableSectionKind::Summoner(ranked_summoner));
                sections.push(DisplayableSectionKind::MatchHistory(match_history));
            }
            Mode::Custom(_) => {}
        }

        Self {
            image: lolfetch_ascii::from_url(&data.image_url, IMAGE_WIDTH, IMAGE_HEIGHT)
                .await
                .unwrap(),
            sections,
        }
    }
}
