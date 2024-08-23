//! This module contains communication with Dragon (DDragon and CommunityDragon) APIs for fetching images.

use riven::{consts::Champion, models::summoner_v4::Summoner};
use tokio::sync::OnceCell;

static ONCE: OnceCell<String> = OnceCell::const_new();

pub async fn get_latest_patch() -> &'static String {
    ONCE.get_or_init(|| async {
        let url = "https://ddragon.leagueoflegends.com/api/versions.json";
        let patch = reqwest::get(url)
            .await
            .unwrap()
            .json::<Vec<String>>()
            .await
            .unwrap();
        patch[0].clone()
    })
    .await
}

pub trait IconGetter {
    /// Returns the icon.
    async fn get_icon_url(&self) -> Option<String>;
}

impl IconGetter for Summoner {
    async fn get_icon_url(&self) -> Option<String> {
        Some(format!(
            "https://cdn.communitydragon.org/{}/profile-icon/{}",
            get_latest_patch().await,
            self.profile_icon_id
        ))
    }
}

impl IconGetter for Champion {
    async fn get_icon_url(&self) -> Option<String> {
        Some(format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
            get_latest_patch().await,
            self
        ))
    }
}
