use lolfetch_color::ColoredString;
use riven::models::league_v4::LeagueEntry;
use termcolor::Color;

use crate::{api::account::RiotId, display::utils::RankColorGetter};

use super::SectionInfoProvider;

pub struct SummonerInfo {
    pub riot_id: RiotId,
    pub ranked: LeagueEntry,
}

impl SummonerInfo {
    pub fn new(riot_id: &RiotId, ranked: LeagueEntry) -> Self {
        Self {
            riot_id: riot_id.clone(),
            ranked,
        }
    }
}

impl SectionInfoProvider for SummonerInfo {
    fn header(&self) -> Option<ColoredString> {
        None
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut vec = Vec::new();
        // Name
        vec.push(ColoredString::from_str(
            &format!("Name: {}", self.riot_id),
            None,
            None,
        ));

        // Rank
        let mut rank_string = ColoredString::new();
        let rank = self.ranked.tier.unwrap_or(riven::consts::Tier::UNRANKED);
        let rank_color = rank.get_rank_color();
        rank_string.push_str("Rank: ", None, None);
        rank_string.push_str(&format!("{rank}"), rank_color, None);
        match self.ranked.rank {
            Some(division) if !rank.is_apex() => {
                rank_string.push_str(" ", None, None);
                rank_string.push_str(&format!("{division}"), rank_color, None);
            }
            _ => {}
        }
        rank_string.push_str(" - ", None, None);
        rank_string.push_str(
            &format!("{} LP", self.ranked.league_points),
            rank_color,
            None,
        );
        vec.push(rank_string);

        // Winrate
        let mut winrate_string = ColoredString::from_str(&" ".repeat(6), None, None);
        let winrate_bar =
            generate_winrate_bar(self.ranked.wins, self.ranked.losses, 30, rank_color);
        winrate_string.join(&winrate_bar);
        winrate_string.push_str(
            &format!(
                " {:.1}%",
                (f64::from(self.ranked.wins) / f64::from(self.ranked.wins + self.ranked.losses))
                    * 100.0
            ),
            rank_color,
            None,
        );
        winrate_string.push_str(
            &format!(
                " ({wins}W/{losses}L)",
                wins = self.ranked.wins,
                losses = self.ranked.losses
            ),
            None,
            None,
        );
        vec.push(winrate_string);

        vec
    }
}

pub fn generate_winrate_bar(
    wins: i32,
    losses: i32,
    width: i32,
    win_color: Option<Color>,
) -> ColoredString {
    let mut winrate_bar = ColoredString::new();
    let total = wins + losses;
    let winrate = if total == 0 {
        0.0
    } else {
        f64::from(wins) / f64::from(total)
    };
    let winrate_width = (winrate * f64::from(width)).round() as i32;
    for i in 0..width {
        winrate_bar.push_str(
            " ",
            None,
            if i < winrate_width {
                win_color
            } else {
                Some(Color::White)
            },
        );
    }
    winrate_bar
}
