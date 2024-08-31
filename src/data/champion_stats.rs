use crate::{
    display::DisplayableSection,
    models::{champion_stats::GameStats, matches::MatchPlayerInfo},
};
use anyhow::Result;
use lolfetch_color::ColoredString;
use riven::{
    consts::Champion,
    models::{match_v5::Match, summoner_v4::Summoner},
};
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
    pub fn new(matches: &[Match], summoner: &Summoner, max_champs: i32) -> Result<Self> {
        let mut stats = HashMap::new();

        for game in matches {
            let match_info = MatchPlayerInfo::from_match(game, summoner)?;
            let champion_stats = stats.entry(match_info.champion).or_insert(GameStats::new());
            champion_stats.add_game(&match_info);
        }

        let mut sorted = stats.into_iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.1.total_games().cmp(&a.1.total_games()));

        Ok(Self {
            stats: sorted
                .into_iter()
                .take(max_champs as usize)
                .map(|(champion, stats)| ChampionStats { champion, stats })
                .collect(),
            games_processed: matches.len(),
        })
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

        for _ in 0..5 {
            let champion_stats = match stat_iter.next() {
                Some(stats) => stats,
                None => break,
            };
            let mut champion_body = ColoredString::new();
            let kda_str = match champion_stats.stats.kda() {
                Some(kda) => format!("{:.2}", kda),
                None => "Perfect".to_string(),
            };

            champion_body.push_unformatted_str(&format!(
                "{:<12} - {:.2}% WR - {:.2} KDA - {:.1} CS/M - {} Played",
                champion_stats.champion.name().unwrap(),
                champion_stats.stats.winrate() * 100.0,
                kda_str,
                champion_stats.stats.cspm(),
                champion_stats.stats.total_games(),
            ));

            body.push(champion_body);
        }

        body
    }
}
