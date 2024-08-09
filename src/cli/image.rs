//! This module handles ASCII art images

use riven::consts::Tier;

/// Associates an image URL to an object
pub trait ImageUrl {
    // TODO: Maybe create a URL type
    fn to_image(&self) -> String;
}

impl ImageUrl for Tier {
    fn to_image(&self) -> String {
        match self {
            Tier::IRON => "https://static.wikia.nocookie.net/leagueoflegends/images/f/f8/Season_2023_-_Iron.png/revision/latest",
            Tier::BRONZE => "https://static.wikia.nocookie.net/leagueoflegends/images/c/cb/Season_2023_-_Bronze.png/revision/latest",
            Tier::SILVER => "https://static.wikia.nocookie.net/leagueoflegends/images/c/c4/Season_2023_-_Silver.png/revision/latest",
            Tier::GOLD => "https://static.wikia.nocookie.net/leagueoflegends/images/7/78/Season_2023_-_Gold.png/revision/latest",
            Tier::PLATINUM => "https://static.wikia.nocookie.net/leagueoflegends/images/b/bd/Season_2023_-_Platinum.png/revision/latest",
            Tier::EMERALD => "https://static.wikia.nocookie.net/leagueoflegends/images/4/4b/Season_2023_-_Emerald.png/revision/latest",
            Tier::DIAMOND => "https://static.wikia.nocookie.net/leagueoflegends/images/3/37/Season_2023_-_Diamond.png/revision/latest",
            Tier::MASTER => "https://static.wikia.nocookie.net/leagueoflegends/images/d/d5/Season_2023_-_Master.png/revision/latest",
            Tier::GRANDMASTER => "https://static.wikia.nocookie.net/leagueoflegends/images/6/64/Season_2023_-_Grandmaster.png/revision/latest",
            Tier::CHALLENGER => "https://static.wikia.nocookie.net/leagueoflegends/images/1/14/Season_2023_-_Challenger.png/revision/latest",
            Tier::UNRANKED => "https://static.wikia.nocookie.net/leagueoflegends/images/3/3e/Season_2022_-_Unranked.png/revision/latest",
        }.to_string()
    }
}
