//! This module contains utilities regarding the display of the application.

use riven::consts::Tier;

pub trait RankColorGetter {
    /// Returns the color associated with the league rank.
    fn get_rank_color(&self) -> Option<termcolor::Color>;
}

impl RankColorGetter for Tier {
    fn get_rank_color(&self) -> Option<termcolor::Color> {
        match self {
            Self::IRON => Some(termcolor::Color::Ansi256(102)), // Dark gray
            Self::BRONZE => Some(termcolor::Color::Ansi256(130)), // Bronze
            Self::SILVER => Some(termcolor::Color::Ansi256(145)), // Silver
            Self::GOLD => Some(termcolor::Color::Ansi256(178)), // Gold
            Self::PLATINUM => Some(termcolor::Color::Ansi256(80)), // Teal
            Self::EMERALD => Some(termcolor::Color::Ansi256(35)), // Emerald green
            Self::DIAMOND => Some(termcolor::Color::Ansi256(69)), // Light blue
            Self::MASTER => Some(termcolor::Color::Ansi256(99)), // Purple
            Self::GRANDMASTER => Some(termcolor::Color::Ansi256(160)), // Red
            Self::CHALLENGER => Some(termcolor::Color::Ansi256(220)), // Light gold
            Self::UNRANKED => None,
        }
    }
}
