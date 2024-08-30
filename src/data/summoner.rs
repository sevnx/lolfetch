//! Provides information about a summoner.

use super::SectionInfoProvider;
use crate::{api::account::RiotId, display::utils::RankColorGetter, models::ranked::RankedInfo};
use lolfetch_color::ColoredString;
use termcolor::Color;

pub struct Info {
    pub riot_id: RiotId,

    /// Rnak information of the summoner.
    /// It is optional because this struct can be used to display information about a summoner
    /// without a rank.
    pub ranked: Option<RankedInfo>,
}

impl Info {
    pub fn new(riot_id: &RiotId, ranked: Option<RankedInfo>) -> Self {
        Self {
            riot_id: riot_id.clone(),
            // Filter out unranked tiers (since it is useless to display them).
            ranked: ranked.filter(|r| r.tier != riven::consts::Tier::UNRANKED),
        }
    }
}

impl SectionInfoProvider for Info {
    fn header(&self) -> Option<String> {
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

        // Early return if the summoner is unranked.
        let ranked_info = match &self.ranked {
            Some(ranked) => ranked,
            None => return vec,
        };

        // Rank
        let mut rank_string = ColoredString::new();

        let rank = ranked_info.tier;
        let rank_color = rank.get_rank_color();
        rank_string.push_str("Rank: ", None, None);
        rank_string.push_str(&format!("{rank}"), rank_color, None);

        match ranked_info.division {
            Some(division) if !rank.is_apex() => {
                rank_string.push_str(" ", None, None);
                rank_string.push_str(&format!("{division}"), rank_color, None);
            }
            _ => {}
        }

        // LP
        rank_string.push_str(" - ", None, None);
        rank_string.push_str(&format!("{} LP", ranked_info.lp), rank_color, None);
        vec.push(rank_string);

        // Winrate
        let mut winrate_string = ColoredString::from_str(&" ".repeat(6), None, None);
        let winrate_bar =
            generate_winrate_bar(ranked_info.wins, ranked_info.losses, 30, rank_color);
        winrate_string.join(&winrate_bar);

        winrate_string.push_str(
            &format!(
                " {:.1}%",
                get_winrate(ranked_info.wins, ranked_info.losses).unwrap() * 100.0
            ),
            rank_color,
            None,
        );

        winrate_string.push_str(
            &format!(
                " ({wins}W/{losses}L)",
                wins = ranked_info.wins,
                losses = ranked_info.losses
            ),
            None,
            None,
        );
        vec.push(winrate_string);

        vec
    }
}

fn get_winrate(wins: i32, losses: i32) -> Option<f64> {
    let total = wins + losses;
    if total == 0 {
        None
    } else {
        Some(f64::from(wins) / f64::from(total))
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
