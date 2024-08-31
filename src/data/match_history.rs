use crate::display::DisplayableSection;
use crate::models::matches::{GameResult, MatchPlayerInfo, MatchPlayerInfoError};
use lolfetch_color::ColoredString;
use riven::models::match_v5::Match;
use riven::models::summoner_v4::Summoner;
use termcolor::Color;

pub struct MatchHistory {
    matches: Vec<MatchPlayerInfo>,
}

impl MatchHistory {
    pub fn new(
        matches: &[Match],
        summoner: &Summoner,
        max_games: i32,
    ) -> Result<Self, MatchPlayerInfoError> {
        let match_infos = matches
            .iter()
            .take(max_games as usize)
            .map(|game| MatchPlayerInfo::from_match(game, summoner))
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

            if let Some(kda) = match_info.kda.get_kda() {
                match_body.push_unformatted_str(&format!("{:.2} KDA", kda));
            } else {
                match_body.push_unformatted_str("Perfect");
            }

            #[allow(clippy::cast_precision_loss)]
            let cspm =
                f64::from(match_info.minions_killed) / (f64::from(match_info.time_played) / 60.0);

            match_body.push_unformatted_str(&format!(" - {cspm:.1} CS/M"));

            body.push(match_body);
        }

        body
    }
}
