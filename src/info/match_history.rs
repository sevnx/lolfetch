use super::SectionInfoProvider;
use crate::utils::Kda;
use anyhow::Result;
use lolfetch_color::{ColoredChar, ColoredString};
use riven::models::{
    match_v5::{Match, Participant, Team},
    summoner_v4::Summoner,
};

pub struct RecentMatchesInfo {
    matches: Vec<Match>,
    summoner: Summoner,
}

impl RecentMatchesInfo {
    pub fn new(matches: Vec<Match>, summoner: &Summoner) -> Self {
        Self {
            matches,
            summoner: summoner.clone(),
        }
    }
}

impl SectionInfoProvider for RecentMatchesInfo {
    fn header(&self) -> Option<ColoredString> {
        Some(ColoredString::from_str("Recent Matches", None, None))
    }

    fn body(&self) -> Vec<ColoredString> {
        self.matches
            .iter()
            .map(|match_data| {
                // Potentially change this
                self.format_match(match_data)
                    .unwrap_or_else(|_| panic!("Failed to format match"))
            })
            .collect()
    }
}

impl RecentMatchesInfo {
    fn format_match(&self, match_data: &Match) -> Result<ColoredString> {
        let participant = match_data.get_participant(&self.summoner)?;
        let my_team = match_data.get_my_team(participant)?;
        let max_time = match_data.get_max_time()?;

        let kda = Kda(participant.kills, participant.deaths, participant.assists);

        let mut body_builder = ColoredString::new();
        // Game result
        body_builder.push(get_game_result_char(my_team.win));
        body_builder.push_str(
            &format!(
                " - {} - {:<12}",
                get_team_position(&participant.team_position),
                participant.champion().unwrap().name().unwrap(),
            ),
            None,
            None,
        );
        body_builder.push_str(&format!(" - {:8} - ", kda.to_string()), None, None);

        // KDA
        if let Some(kda) = kda.get_kda() {
            body_builder.push_str(&format!("{kda:.2} KDA"), None, None);
        } else {
            body_builder.push_str("Perfect", None, None);
        }

        // CS per minute
        let (start, end) = (
            match_data.info.game_start_timestamp,
            match_data.info.game_start_timestamp + i64::from(max_time),
        );
        let duration = end - start;
        let total_minions = participant.total_minions_killed + participant.neutral_minions_killed;
        #[allow(clippy::cast_precision_loss)] // This is fine as duration is pretty small
        let cspm = f64::from(total_minions) / (duration as f64 / 60.0);

        body_builder.push_str(&format!(" - {cspm:.1} CS/M"), None, None);
        Ok(body_builder)
    }
}

/// Returns the team position in a short form
fn get_team_position(position: &str) -> &str {
    match position {
        "TOP" => "TOP",
        "JUNGLE" => "JGL",
        "MIDDLE" => "MID",
        "BOTTOM" => "BOT",
        "UTILITY" => "SUP",
        _ => position,
    }
}

/// Returns a colored character representing the game result
const fn get_game_result_char(won: bool) -> ColoredChar {
    if won {
        ColoredChar::new('W', Some(termcolor::Color::Blue), None)
    } else {
        ColoredChar::new('L', Some(termcolor::Color::Red), None)
    }
}

pub trait ParticipantGetter {
    fn get_participant(&self, summoner: &Summoner) -> Result<&Participant>;
}

impl ParticipantGetter for Match {
    fn get_participant(&self, summoner: &Summoner) -> Result<&Participant> {
        self.info
            .participants
            .iter()
            .find(|p| p.puuid == summoner.puuid)
            .ok_or_else(|| anyhow::anyhow!("Failed to find participant"))
    }
}

pub trait TeamGetter {
    fn get_my_team(&self, participant: &Participant) -> Result<&Team>;
}

impl TeamGetter for Match {
    fn get_my_team(&self, participant: &Participant) -> Result<&Team> {
        self.info
            .teams
            .iter()
            .find(|t| t.team_id == participant.team_id)
            .ok_or_else(|| anyhow::anyhow!("Failed to find team"))
    }
}

pub trait GameTimeGetter {
    fn get_max_time(&self) -> Result<i32>;
}

impl GameTimeGetter for Match {
    fn get_max_time(&self) -> Result<i32> {
        self.info
            .participants
            .iter()
            .map(|p| p.time_played)
            .max()
            .ok_or_else(|| anyhow::anyhow!("Failed to find max time"))
    }
}
