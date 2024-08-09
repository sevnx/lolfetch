use crate::cli::image::ImageUrl;
use crate::riot_api::account::{PuuidGetter, RiotId};
use anyhow::Result;
use cli::Layout;
use lolfetch_ascii::ascii_art;
use riot_api::ApiInstanceGetter;
use riven::{consts::QueueType, RiotApi};

mod cli;
mod riot_api;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let api = RiotApi::get_api_instance()?;

    let account = api.get_puuid(&RiotId::new("sev", "cat")).await?;
    println!("Account: {}", account);

    let summoner = api
        .summoner_v4()
        .get_by_puuid(riven::consts::PlatformRoute::EUW1, &account)
        .await?;
    println!("Summoner: {:?}", summoner);

    for entry in api
        .league_v4()
        .get_league_entries_for_summoner(riven::consts::PlatformRoute::EUW1, &summoner.id)
        .await?
    {
        if entry.queue_type == QueueType::RANKED_SOLO_5x5 {
            let rank = entry.tier.unwrap_or(riven::consts::Tier::UNRANKED);
            println!("Trying to display rank: {:?}", rank);
            let art = ascii_art::from_url(&rank.to_image(), 50, 25)
                .await
                .unwrap_or_else(|err| {
                    panic!("Error: {}", err);
                });
            println!("Trying to display art");
            Layout::new(art, vec![]).display();
        }
    }

    Ok(())
}
