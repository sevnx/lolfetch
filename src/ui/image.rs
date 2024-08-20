//! This module handles ASCII art images

use riven::consts::Tier;

/// Associates an image URL to an object
pub trait ImgUrlGetter {
    // TODO: Maybe create a URL type
    fn get_image_url(&self) -> String;
}

impl ImgUrlGetter for Tier {
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
