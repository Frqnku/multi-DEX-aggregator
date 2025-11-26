use ethers::prelude::*;
use std::sync::Arc;

use crate::config::Config;
mod aggregator;
mod config;
mod dex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_file("config.json")?;

    let provider = Provider::<Http>::try_from(cfg.rpc_url.unwrap())?;
    let provider = Arc::new(provider);

    for token in &cfg.tokens {
        let price = aggregator::compute_token_price(provider.clone(), token).await?;
        println!("Token {} : {:.6} USD", token.name, price);
    }

    Ok(())
}
