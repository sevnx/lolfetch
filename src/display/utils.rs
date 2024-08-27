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

pub trait ImageUrlGetter {
    // TODO: Maybe create a URL type
    fn get_image_url(&self) -> String;
}

impl ImageUrlGetter for Tier {
    fn get_image_url(&self) -> String {
        match self {
            Self::IRON => "https://static.wikia.nocookie.net/leagueoflegends/images/f/f8/Season_2023_-_Iron.png/revision/latest",
            Self::BRONZE => "https://static.wikia.nocookie.net/leagueoflegends/images/c/cb/Season_2023_-_Bronze.png/revision/latest",
            Self::SILVER => "https://static.wikia.nocookie.net/leagueoflegends/images/c/c4/Season_2023_-_Silver.png/revision/latest",
            Self::GOLD => "https://static.wikia.nocookie.net/leagueoflegends/images/7/78/Season_2023_-_Gold.png/revision/latest",
            Self::PLATINUM => "https://static.wikia.nocookie.net/leagueoflegends/images/b/bd/Season_2023_-_Platinum.png/revision/latest",
            Self::EMERALD => "https://static.wikia.nocookie.net/leagueoflegends/images/4/4b/Season_2023_-_Emerald.png/revision/latest",
            Self::DIAMOND => "https://static.wikia.nocookie.net/leagueoflegends/images/3/37/Season_2023_-_Diamond.png/revision/latest",
            Self::MASTER => "https://static.wikia.nocookie.net/leagueoflegends/images/d/d5/Season_2023_-_Master.png/revision/latest",
            Self::GRANDMASTER => "https://static.wikia.nocookie.net/leagueoflegends/images/6/64/Season_2023_-_Grandmaster.png/revision/latest",
            Self::CHALLENGER => "https://static.wikia.nocookie.net/leagueoflegends/images/1/14/Season_2023_-_Challenger.png/revision/latest",
            Self::UNRANKED => "https://static.wikia.nocookie.net/leagueoflegends/images/3/3e/Season_2022_-_Unranked.png/revision/latest",
        }.to_string()
    }
}
