use lolfetch_color::ColoredString;
use riven::models::champion_mastery_v4::ChampionMastery;

use super::SectionInfoProvider;

pub struct MasteryInfo {
    masteries: Vec<ChampionMastery>,
}

impl MasteryInfo {
    pub fn new(masteries: Vec<ChampionMastery>) -> Self {
        Self { masteries }
    }
}

impl SectionInfoProvider for MasteryInfo {
    fn header(&self) -> Option<String> {
        Some("Champion Masteries".to_string())
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut vec = Vec::new();
        for (i, mastery) in self.masteries.iter().enumerate() {
            let mastery_str = format!(
                "{}. {:<12} - Level {} - {} points",
                i + 1,
                mastery.champion_id.name().unwrap(),
                mastery.champion_level,
                mastery.champion_points
            );
            vec.push(ColoredString::from_str(&mastery_str, None, None));
        }
        vec
    }
}
