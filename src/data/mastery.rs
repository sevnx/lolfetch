use lolfetch_color::ColoredString;
use riven::models::champion_mastery_v4::ChampionMastery;

use crate::display::DisplayableSection;

pub struct Mastery {
    masteries: Vec<ChampionMastery>,
}

impl Mastery {
    pub fn new(masteries: Vec<ChampionMastery>, max_champs: i32) -> Self {
        Self {
            masteries: masteries.into_iter().take(max_champs as usize).collect(),
        }
    }
}

impl DisplayableSection for Mastery {
    fn header(&self) -> Option<String> {
        Some("Champion Mastery".to_string())
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut body = Vec::new();
        for (i, mastery) in self.masteries.iter().enumerate() {
            let mastery_str = format!(
                "{}. {:<12} - Level {} - {} points",
                i + 1,
                mastery.champion_id.name().unwrap(),
                mastery.champion_level,
                mastery.champion_points
            );
            body.push(ColoredString::from_unformatted_str(&mastery_str));
        }
        body
    }
}
