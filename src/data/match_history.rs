use crate::display::DisplayableSection;
use crate::models::champion_stats::GameStats;
use crate::models::matches::{GameResult, MatchPlayerInfo, MatchPlayerInfoError};
use lolfetch_color::ColoredString;
use riven::models::match_v5;
use riven::models::summoner_v4::Summoner;
use termcolor::Color;

pub struct MatchHistory {
    matches: Vec<MatchPlayerInfo>,
}

impl MatchHistory {
    pub fn new(
        matches: &[match_v5::Info],
        summoner: &Summoner,
        max_games: i32,
    ) -> Result<Self, MatchPlayerInfoError> {
        let match_infos = matches
            .iter()
            .take(max_games as usize)
            .map(|game| MatchPlayerInfo::from_match_info(game, summoner))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            matches: match_infos,
        })
    }
}

impl DisplayableSection for MatchHistory {
    fn header(&self) -> Option<String> {
        Some("Match History".to_string())
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut body = Vec::new();

        for match_info in &self.matches {
            let mut match_body = ColoredString::new();

            match match_info.game_result {
                GameResult::Win => {
                    match_body.push_str("W", Some(Color::Blue), None);
                }
                GameResult::Loss => {
                    match_body.push_str("L", Some(Color::Red), None);
                }
            }

            match_body.push_unformatted_str(&format!(
                " - {} - {:<12}",
                match_info.team_position,
                match_info.champion.name().unwrap()
            ));

            match_body.push_unformatted_str(&format!(" - {:8} - ", match_info.kda.to_string()));

            let game_stats = GameStats::from(match_info);

            if let Some(kda) = game_stats.kda() {
                match_body.push_unformatted_str(&format!("{kda:.2} KDA"));
            } else {
                match_body.push_unformatted_str("PERFECT");
            }

            match_body.push_unformatted_str(&format!(" - {:.1} CS/M", game_stats.cspm()));

            body.push(match_body);
        }

        body
    }
}
