//! Provides information about a summoner.

use crate::{
    api::account::RiotId,
    display::{
        utils::{colors::RankColorGetter, generate_loading_bar},
        DisplayableSection,
    },
    models::ranked::RankedInfo,
};
use lolfetch_color::ColoredString;
use riven::models::league_v4::LeagueEntry;

/// Summoner information.
pub struct Summoner {
    pub riot_id: RiotId,

    /// Rnak information of the summoner.
    /// It is optional because this struct can be used to display information about a summoner
    /// without a rank.
    pub ranked: Option<RankedInfo>,
}

impl Summoner {
    pub fn new(riot_id: &RiotId, ranked: Option<LeagueEntry>) -> Self {
        let ranked = ranked.and_then(RankedInfo::from_entry);

        Self {
            riot_id: riot_id.clone(),
            // Filter out unranked tiers (since it is useless to display them).
            ranked: ranked.filter(|r| r.tier != riven::consts::Tier::UNRANKED),
        }
    }
}

impl DisplayableSection for Summoner {
    fn header(&self) -> Option<String> {
        None
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut body = Vec::new();

        // Summoner name
        body.push(ColoredString::from_unformatted_str(&format!(
            "Summoner: {}",
            self.riot_id
        )));

        if let Some(ranked) = &self.ranked {
            // Rank / LP
            let mut ranked_string = ColoredString::new();
            let rank_color = ranked.tier.get_rank_color().unwrap();
            ranked_string.push_unformatted_str("Rank: ");
            ranked_string.push_str(&format!("{}", ranked.tier), Some(rank_color), None);
            match ranked.division {
                Some(division) if !ranked.tier.is_apex() => {
                    ranked_string.push_str(&format!(" {division}"), Some(rank_color), None);
                }
                _ => {}
            }
            ranked_string.push_str(&format!(" - {} LP", ranked.lp), Some(rank_color), None);

            // Winrate bar
            let mut winrate_string = ColoredString::from_unformatted_str(&" ".repeat(6));
            const WINRATE_BAR_WIDTH: i32 = 30;
            winrate_string.join(&generate_loading_bar(
                ranked.wins,
                ranked.losses,
                WINRATE_BAR_WIDTH,
                rank_color,
            ));

            winrate_string.push_str(
                &format!(" {:.1}%", ranked.get_winrate().unwrap(),),
                Some(rank_color),
                None,
            );
            winrate_string.push_unformatted_str(&format!(" ({}W/{}L)", ranked.wins, ranked.losses));

            body.push(ranked_string);
        }

        body
    }
}
