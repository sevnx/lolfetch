//! This module contains communication with static data from
//! - [Data Dragon](https://developer.riotgames.com/docs/lol#data-dragon)
//! - [Community Dragon](https://communitydragon.org/)

use riven::{
    consts::{Champion, Tier},
    models::summoner_v4::Summoner,
};
use tokio::sync::OnceCell;

static ONCE: OnceCell<String> = OnceCell::const_new();

pub async fn get_latest_patch() -> &'static String {
    ONCE.get_or_init(|| async {
        let url = "https://ddragon.leagueoflegends.com/api/versions.json";
        let patch = reqwest::get(url)
            .await
            .unwrap_or_else(|_| panic!("Failed to fetch patch version from {url}"))
            .json::<Vec<String>>()
            .await
            .unwrap_or_else(|_| panic!("Failed to parse patch version from {url}"));
        patch[0].clone()
    })
    .await
}

pub trait IconGetter {
    /// Returns the icon.
    async fn get_icon_url(&self) -> String;
}

impl IconGetter for Summoner {
    async fn get_icon_url(&self) -> String {
        format!(
            "https://cdn.communitydragon.org/{}/profile-icon/{}",
            get_latest_patch().await,
            self.profile_icon_id
        )
    }
}

impl IconGetter for Champion {
    async fn get_icon_url(&self) -> String {
        format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
            get_latest_patch().await,
            self
        )
    }
}

impl IconGetter for Tier {
    async fn get_icon_url(&self) -> String {
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
