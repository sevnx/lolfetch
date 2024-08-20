use anyhow::Result;
use clap::Parser;
use cli::Options;
use info::match_history::RecentMatchesInfo;
use info::summoner::SummonerInfo;
use info::Sections;
use riot_api::account::{PuuidGetter, RiotId};
use riot_api::matches::MatchGetter;
use riot_api::rank::RankRetriever;
use riot_api::ApiInstanceGetter;
use riven::consts::Queue;
use riven::RiotApi;
use ui::image::ImgUrlGetter;
use ui::Layout;

mod cli;
mod info;
mod riot_api;
mod ui;

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

    let rank = ranked_entry.tier.unwrap_or(riven::consts::Tier::UNRANKED);

    let image_url = args.path.map_or_else(|| rank.get_image_url(), |path| path);

    let matches = summoner
        .get_recent_matches(
            api,
            route.to_regional(),
            5,
            Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO,
        )
        .await?;

    let art = lolfetch_ascii::from_url(&image_url, 50, 25)
        .await
        .unwrap_or_else(|err| {
            panic!("Error: {err}");
        });

    let mut info_vec = Vec::new();

    info_vec.push(Sections::SummonerInfo(SummonerInfo::new(
        &riot_id,
        ranked_entry,
    )));
    info_vec.push(Sections::RecentMatchesInfo(RecentMatchesInfo::new(
        matches, &summoner,
    )));

    Layout::new(art, info_vec).display()
}
