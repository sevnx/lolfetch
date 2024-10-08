use crate::{
    display::DisplayableSection,
    models::{
        champion_stats::GameStats,
        matches::{MatchInfo, MatchPlayerInfo},
    },
};
use lolfetch_color::ColoredString;
use riven::{consts::Champion, models::summoner_v4::Summoner};
use std::collections::HashMap;

pub struct ChampionStats {
    champion: Champion,
    stats: GameStats,
}

pub struct RecentChampionInfo {
    stats: Vec<ChampionStats>,
    games_processed: usize,
}

impl RecentChampionInfo {
    pub fn new(matches: &[MatchInfo], summoner: &Summoner, max_champs: i32) -> Self {
        let mut stats = HashMap::new();

        for game in matches {
            let match_info = MatchPlayerInfo::from_match_info(game, summoner)
                .expect("Failed to get match player info");
            let champion_stats = stats.entry(match_info.champion).or_insert(GameStats::new());
            champion_stats.add_game(&match_info);
        }

        let mut sorted = stats.into_iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.1.total_games().cmp(&a.1.total_games()));

        Self {
            stats: sorted
                .into_iter()
                .take(max_champs as usize)
                .map(|(champion, stats)| ChampionStats { champion, stats })
                .collect(),
            games_processed: matches.len(),
        }
    }

    pub fn max_champion_name_width(&self) -> Option<usize> {
        self.stats
            .iter()
            .map(|s| {
                s.champion
                    .name()
                    .expect("Failed to get champion name")
                    .len()
            })
            .max()
    }
}

impl DisplayableSection for RecentChampionInfo {
    fn header(&self) -> Option<String> {
        Some(format!(
            "Champion Stats (last {} games)",
            self.games_processed
        ))
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut body = Vec::new();

        let mut stat_iter = self.stats.iter();

        let max_width = self.max_champion_name_width().expect("No matches found");

        for _ in 0..5 {
            let Some(champion_stats) = stat_iter.next() else {
                break;
            };
            let mut champion_body = ColoredString::new();
            let kda_str = champion_stats
                .stats
                .kda()
                .map_or_else(|| "PERFECT".to_string(), |kda| format!("{kda:.1} KDA"));

            champion_body.push_unformatted_str(&format!(
                "{:<width$} - {:3.0}% WR - {} - {:.1} CS/M - {} Played",
                champion_stats
                    .champion
                    .name()
                    .expect("Failed to get champion name"),
                champion_stats.stats.winrate() * 100.0,
                kda_str,
                champion_stats.stats.cspm(),
                champion_stats.stats.total_games(),
                width = max_width
            ));

            body.push(champion_body);
        }

        body
    }
}
