use crate::dex::{DexProtocol, decimals_for_stablecoin};

use std::sync::Arc;

use anyhow::Result;
use ethers::{
    abi::Address,
    contract::abigen,
    providers::{Http, Provider},
};

pub struct Sushiswap;

abigen!(
    SushiswapPair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32)
        function token0() external view returns (address)
        function token1() external view returns (address)
    ]"#
);

#[async_trait::async_trait]
impl DexProtocol for Sushiswap {
    async fn get_price_and_tvl(
        &self,
        provider: Arc<Provider<Http>>,
        pool: Address,
        token: Address,
    ) -> Result<(f64, f64)> {
        let contract = SushiswapPair::new(pool, provider);

        let (reserve0, reserve1, _) = contract.get_reserves().call().await?;
        let token0 = contract.token_0().call().await?;
        let token1 = contract.token_1().call().await?;

        let (reserve_t0, reserve_t1, stablecoin) = if token0 == token {
            (reserve0, reserve1, token1)
        } else if token1 == token {
            (reserve1, reserve0, token0)
        } else {
            panic!("Token not found in pool");
        };
        let reserve_t0_f = reserve_t0 as f64 / 1e18;
        let reserve_t1_f = reserve_t1 as f64 / decimals_for_stablecoin(stablecoin);

        let price = reserve_t1_f / reserve_t0_f;
        let tvl = reserve_t0_f * price + reserve_t1_f;

        Ok((price, tvl))
    }
}
