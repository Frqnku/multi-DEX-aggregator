use anyhow::Result;
use ethers::prelude::*;
use futures::future::join_all;
use std::sync::Arc;

use crate::config::{Config, TokenConfig};
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_file("config.json")?;

    let provider = Provider::<Http>::try_from(cfg.rpc_url.unwrap())?;
    let provider = Arc::new(provider);

    abigen!(
        UniswapV2Pair,
        r#"[
            function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32)
            function token0() external view returns (address)
            function token1() external view returns (address)
        ]"#
    );

    async fn get_price_and_tvl(
        provider: Arc<Provider<Http>>,
        pool: Address,
        token: Address,
    ) -> anyhow::Result<(f64, f64)> {
        let contract = UniswapV2Pair::new(pool, provider);
        let (reserve0, reserve1, _) = contract.get_reserves().call().await?;
        let token0 = contract.token_0().call().await?;
        let token1 = contract.token_1().call().await?;

        let (reserve_t0, reserve_t1) = if token0 == token {
            (reserve0, reserve1)
        } else if token1 == token {
            (reserve1, reserve0)
        } else {
            panic!("Token not found in pool");
        };
        let reserve_t0_f = reserve_t0 as f64 / 1e18;
        let reserve_t1_f = reserve_t1 as f64 / 1e6;

        let price = reserve_t1_f / reserve_t0_f;
        let tvl = reserve_t0_f * price + reserve_t1_f;
        Ok((price, tvl))
    }

    pub async fn compute_token_price(
        provider: Arc<Provider<Http>>,
        token: &TokenConfig,
    ) -> Result<f64> {
        let futures = token.pools.iter().map(|pool| {
            let provider = provider.clone();
            let pool_addr: Address = pool.address.parse().unwrap();
            let token_addr: Address = token.token.parse().unwrap();

            async move { get_price_and_tvl(provider.clone(), pool_addr, token_addr).await }
        });

        let results: Vec<Result<(f64, f64)>> = join_all(futures).await;

        let valid = results
            .into_iter()
            .inspect(|e| eprintln!("Erreur: {e:?}"))
            .filter_map(|res| res.ok())
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

    for token in &cfg.tokens {
        let price = compute_token_price(provider.clone(), token).await?;
        println!("Token {} : {:.6} USD", token.name, price);
    }

    Ok(())
}
