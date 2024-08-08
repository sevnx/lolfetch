use anyhow::Result;

mod riot_api;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    Ok(())
}
