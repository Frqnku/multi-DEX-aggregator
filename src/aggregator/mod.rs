use anyhow::Result;
use ethers::prelude::*;
use futures::future::join_all;
use std::sync::Arc;

use crate::{
    config::TokenConfig,
    dex::{DexProtocol, sushiswap::Sushiswap, uniswap_v2::UniswapV2, uniswap_v3::UniswapV3},
};

pub async fn compute_token_price(
    provider: Arc<Provider<Http>>,
    token: &TokenConfig,
) -> Result<f64> {
    let futures = token.pools.iter().map(|pool| {
        let provider = provider.clone();
        let pool_addr: Address = pool.address.parse().unwrap();
        let token_addr: Address = token.token.parse().unwrap();

        async move {
            match pool.protocol.as_str() {
                "uniswap_v2" => {
                    UniswapV2
                        .get_price_and_tvl(provider.clone(), pool_addr, token_addr)
                        .await
                }
                "sushiswap" => {
                    Sushiswap
                        .get_price_and_tvl(provider.clone(), pool_addr, token_addr)
                        .await
                }
                "uniswap_v3" => {
                    UniswapV3
                        .get_price_and_tvl(provider.clone(), pool_addr, token_addr)
                        .await
                }
                _ => panic!("Unsupported DEX protocol"),
            }
        }
    });

    let results: Vec<Result<(f64, f64)>> = join_all(futures).await;

    let valid = results
        .into_iter()
        .filter_map(|res| res.inspect_err(|e| eprintln!("Error: {e:?}")).ok())
        .collect::<Vec<_>>();

    if valid.is_empty() {
        anyhow::bail!("No pool returned valid data for token {}", token.name);
    }

    let mut total_tvl = 0.0;
    let mut weighted_sum = 0.0;

    for (price, tvl) in valid {
        total_tvl += tvl;
        weighted_sum += price * tvl;
    }

    let weighted_price = weighted_sum / total_tvl;

    Ok(weighted_price)
}
