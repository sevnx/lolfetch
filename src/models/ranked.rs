use riven::{
    consts::{Division, QueueType, Tier},
    models::league_v4::LeagueEntry,
};

#[derive(Debug)]
pub struct RankedInfo {
    #[allow(unused)] // TODO: Check if useful
    pub queue: QueueType,
    pub tier: Tier,
    pub division: Option<Division>,
    pub lp: i32,
    pub wins: i32,
    pub losses: i32,
}

impl RankedInfo {
    /// Tries to create a `RankedInfo` from a `LeagueEntry`.
    /// Returns `None` if the queue type is not `RANKED_SOLO_5x5` or `RANKED_FLEX_SR`.
    pub fn from_entry(entry: LeagueEntry) -> Option<Self> {
        match entry.queue_type {
            QueueType::RANKED_SOLO_5x5 | QueueType::RANKED_FLEX_SR => {}
            _ => return None,
        }
        Some(Self {
            queue: entry.queue_type,
            tier: entry.tier.unwrap_or(Tier::UNRANKED),
            division: entry.rank,
            lp: entry.league_points,
            wins: entry.wins,
            losses: entry.losses,
        })
    }

    pub fn get_winrate(&self) -> Option<f32> {
        let total = self.wins + self.losses;
        if total == 0 {
            return None;
        }
        Some(self.wins as f32 / total as f32)
    }
}
