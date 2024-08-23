use anyhow::Result;
use clap::Parser;
use cli::{ImageDisplay, Options};
use dragon::{get_latest_patch, IconGetter};
use info::champions::RecentChampionInfo;
use info::mastery::MasteryInfo;
use info::match_history::RecentMatchesInfo;
use info::summoner::SummonerInfo;
use info::Sections;
use riot_api::account::{PuuidGetter, RiotId};
use riot_api::mastery::MasteryRetriever;
use riot_api::matches::MatchGetter;
use riot_api::rank::RankRetriever;
use riot_api::ApiInstanceGetter;
use riven::consts::Queue;
use riven::RiotApi;
use ui::image::ImgUrlGetter;
use ui::Layout;

mod cli;
mod dragon;
mod info;
mod riot_api;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let args = Options::parse();

    let api = RiotApi::get_api_instance()?;
    let riot_id = RiotId::from_str(&args.riot_id)?;
    let account = api.get_puuid(&riot_id).await?;
    let route = args.server.into();
    let summoner = api.summoner_v4().get_by_puuid(route, &account).await?;

    let ranked_entry = summoner.get_rank(&api, route).await?;

    let patch = get_latest_patch().await;

    let rank = ranked_entry.tier.unwrap_or(riven::consts::Tier::UNRANKED);

    let matches = summoner
        .get_recent_matches(
            &api,
            route.to_regional(),
            args.matches,
            Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO,
        )
        .await?;

    let masteries = summoner.get_mastery(&api, route, 5).await?;

    let image_url = match args.display {
        ImageDisplay::Rank => rank.get_image_url(),
        ImageDisplay::Mastery => masteries
            .get(0)
            .unwrap()
            .champion_id
            .get_icon_url()
            .await
            .unwrap_or_else(|| {
                format!("https://cdn.communitydragon.org/{patch}/champion/generic/square")
            }),
        ImageDisplay::Icon => summoner.get_icon_url().await.unwrap_or_else(|| {
            format!("https://cdn.communitydragon.org/{patch}/champion/generic/square")
        }),
        ImageDisplay::Custom => args.path.unwrap_or_else(|| {
            format!("https://cdn.communitydragon.org/{patch}/champion/generic/square",)
        }),
    };

    let art = lolfetch_ascii::from_url(&image_url, 50, 25)
        .await
        .unwrap_or_else(|err| {
            panic!("Error: {err}");
        });

    let info_vec = vec![
        Sections::SummonerInfo(SummonerInfo::new(&riot_id, ranked_entry)),
        Sections::RecentMatchesInfo(RecentMatchesInfo::new(matches.clone(), &summoner)),
        Sections::RecentChampionInfo(
            RecentChampionInfo::from_matches(&matches, &summoner).unwrap(),
        ),
        // Sections::MasteryInfo(MasteryInfo::new(masteries)),
    ];

    Layout::new(art, info_vec).display()
}
