use super::matches::{GameResult, Kda, MatchPlayerInfo};

/// Champion statistics
pub struct GameStats {
    wins: u32,
    losses: u32,
    time_played: i64,
    minions_killed: i32,
    kda: Kda,
}

impl GameStats {
    pub const fn new() -> Self {
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

    pub const fn total_games(&self) -> u32 {
        self.wins + self.losses
    }

    pub fn cspm(&self) -> f64 {
        f64::from(self.minions_killed) / (self.time_played as f64 / 60.0)
    }

    pub fn add_game(&mut self, match_info: &MatchPlayerInfo) {
        self.kda.add(&match_info.kda);
        self.time_played += i64::from(match_info.time_played);
        self.minions_killed += match_info.minions_killed;
        match match_info.game_result {
            GameResult::Win => self.wins += 1,
            GameResult::Loss => self.losses += 1,
        }
    }
}

impl From<&MatchPlayerInfo> for GameStats {
    fn from(match_info: &MatchPlayerInfo) -> Self {
        let mut stats = Self::new();
        stats.add_game(match_info);
        stats
    }
}
