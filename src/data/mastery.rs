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

    pub fn max_champion_name_width(&self) -> Option<usize> {
        self.masteries
            .iter()
            .map(|m| {
                m.champion_id
                    .name()
                    .expect("Failed to get champion name")
                    .len()
            })
            .max()
    }
}

impl DisplayableSection for Mastery {
    fn header(&self) -> Option<String> {
        Some("Champion Mastery".to_string())
    }

    fn body(&self) -> Vec<ColoredString> {
        let mut body = Vec::new();
        let max_width = self.max_champion_name_width().expect("No masteries found");
        for (i, mastery) in self.masteries.iter().enumerate() {
            let mastery_str = format!(
                "{}. {:<width$} - Level {} - {} points",
                i + 1,
                mastery
                    .champion_id
                    .name()
                    .expect("Failed to get champion name"),
                mastery.champion_level,
                mastery.champion_points,
                width = max_width
            );
            body.push(ColoredString::from_unformatted_str(&mastery_str));
        }
        body
    }
}
