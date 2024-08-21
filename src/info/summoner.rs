use lolfetch_ascii::color::ColoredString;
use riven::models::league_v4::LeagueEntry;

use crate::{riot_api::account::RiotId, ui::color::RankColorGetter};

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
        ));
        // Rank
        let mut rank_string = ColoredString::new();
        let rank = self.ranked.tier.unwrap_or(riven::consts::Tier::UNRANKED);
        let rank_color = rank.get_rank_color();
        rank_string.push_str("Rank: ", None);
        rank_string.push_str(&format!("{rank}"), rank_color);
        match self.ranked.rank {
            Some(division) if !rank.is_apex() => {
                rank_string.push_str(" ", None);
                rank_string.push_str(&format!("{division}"), rank_color);
            }
            _ => {}
        }
        rank_string.push_str(" - ", None);
        rank_string.push_str(&format!("{} LP", self.ranked.league_points), rank_color);

        vec.push(rank_string);

        vec
    }
}
