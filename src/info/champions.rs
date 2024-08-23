use super::{
    match_history::{GameTimeGetter, ParticipantGetter, TeamGetter},
    SectionInfoProvider,
};
use crate::utils::Kda;
use anyhow::Result;
use lolfetch_ascii::color::ColoredString;
use riven::{
    consts::Champion,
    models::{match_v5::Match, summoner_v4::Summoner},
};
use std::collections::HashMap;

struct ChampionRankedInfo {
    /// Wins on the champion.
    wins: u32,

    /// Losses on the champion.
    losses: u32,

    /// Total time (seconds) played on the champion (addition of all games).
    time_played: i64,

    /// Total minions killed on the champion.
    minions_killed: i32,

    /// KDAs on the champion.
    kda: Kda,
}

impl ChampionRankedInfo {
    pub fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            time_played: 0,
            minions_killed: 0,
            kda: Kda(0, 0, 0),
        }
    }

    pub fn winrate(&self) -> f32 {
        self.wins as f32 / (self.wins + self.losses) as f32
    }

    pub fn kda(&self) -> Option<f64> {
        self.kda.get_kda()
    }

    pub fn add_game(&mut self, won: bool, time_played: i32, minions_killed: i32, kda: Kda) {
        if won {
            self.wins += 1;
        } else {
            self.losses += 1;
        }

        self.time_played += i64::from(time_played);
        self.minions_killed += minions_killed;
        self.kda.add(&kda);
    }
}

pub struct RecentChampionInfo {
    stats: HashMap<Champion, ChampionRankedInfo>,
}

impl RecentChampionInfo {
    pub fn from_matches(matches: &[Match], summoner: &Summoner) -> Result<Self> {
        let mut stats = HashMap::new();

        for game in matches {
            let participant = game.get_participant(&summoner)?;
            let champion = participant.champion()?;
            let my_team = game.get_my_team(participant)?;
            let time_played = game.get_max_time()?;
            let minions_killed = participant.total_minions_killed;
            let kda = Kda(participant.kills, participant.deaths, participant.assists);

            let champion_stats = stats.entry(champion).or_insert(ChampionRankedInfo::new());

            champion_stats.add_game(my_team.win, time_played, minions_killed, kda);
        }

        Ok(Self { stats })
    }
}

impl SectionInfoProvider for RecentChampionInfo {
    fn header(&self) -> Option<ColoredString> {
        Some(ColoredString::from_str("Recent Champions", None, None))
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut vec = Vec::new();

        for (champion, stats) in &self.stats {
            let winrate = stats.winrate();
            let kda = stats.kda().unwrap_or(0.0);

            let champion_str = format!(
                "{:<12} - {:.1}% winrate - {:.2} KDA - {} minions",
                champion.name().unwrap(),
                winrate * 100.0,
                kda,
                stats.minions_killed
            );

            vec.push(ColoredString::from_str(&champion_str, None, None));
        }

        vec
    }
}
