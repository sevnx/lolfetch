use crate::display::DisplayableSection;
use crate::models::champion_stats::GameStats;
use crate::models::matches::{GameResult, MatchInfo, MatchPlayerInfo};
use lolfetch_color::ColoredString;
use riven::models::summoner_v4::Summoner;
use termcolor::Color;

pub struct MatchHistory {
    matches: Vec<MatchPlayerInfo>,
}

impl MatchHistory {
    pub fn new(matches: &[MatchInfo], summoner: &Summoner, max_games: i32) -> Self {
        let match_infos = matches
            .iter()
            .take(max_games as usize)
            .map(|game| {
                MatchPlayerInfo::from_match_info(game, summoner).expect(
                    "
            Failed to get match player info",
                )
            })
            .collect::<Vec<_>>();

        Self {
            matches: match_infos,
        }
    }

    pub fn max_champion_name_width(&self) -> Option<usize> {
        self.matches
            .iter()
            .map(|m| {
                m.champion
                    .name()
                    .expect("Failed to get champion name")
                    .len()
            })
            .max()
    }
}

impl DisplayableSection for MatchHistory {
    fn header(&self) -> Option<String> {
        Some("Match History".to_string())
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut body = Vec::new();

        let max_width = self.max_champion_name_width().expect("No matches found");

        for match_info in &self.matches {
            let mut match_body = ColoredString::new();

            match_body.push_unformatted_str(&format!(
                "{:02}:{:02} - ",
                match_info.time_played / 60,
                match_info.time_played % 60
            ));

            match match_info.game_result {
                GameResult::Win => {
                    match_body.push_str("W", Some(Color::Blue), None);
                }
                GameResult::Loss => {
                    match_body.push_str("L", Some(Color::Red), None);
                }
            }

            match_body.push_unformatted_str(&format!(
                " - {} - {:<width$}",
                match_info.team_position,
                match_info
                    .champion
                    .name()
                    .expect("Failed to get champion name"),
                width = max_width
            ));

            match_body.push_unformatted_str(&format!(" - {:8} - ", match_info.kda.to_string()));

            let game_stats = GameStats::from(match_info);

            if let Some(kda) = game_stats.kda() {
                match_body.push_unformatted_str(&format!("{kda:.1} KDA"));
            } else {
                match_body.push_unformatted_str("PERFECT");
            }

            match_body.push_unformatted_str(&format!(" - {:.1} CS/M", game_stats.cspm()));

            if let Some(gold_diff) = match_info.gold_diff_15 {
                let color = if gold_diff > 0 {
                    Some(Color::Green)
                } else {
                    Some(Color::Red)
                };

                match_body.push_unformatted_str(" - GD@15: ");
                if gold_diff > 0 {
                    match_body.push_str("+", color, None);
                }
                match_body.push_str(&gold_diff.to_string(), color, None);
            }

            body.push(match_body);
        }

        body
    }
}
