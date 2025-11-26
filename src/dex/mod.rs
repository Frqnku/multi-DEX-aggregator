use std::sync::Arc;

use anyhow::Result;
use ethers::{
    abi::Address,
    providers::{Http, Provider},
};

pub mod sushiswap;
pub mod uniswap_v2;
pub mod uniswap_v3;

#[async_trait::async_trait]
pub trait DexProtocol {
    async fn get_price_and_tvl(
        &self,
        provider: Arc<Provider<Http>>,
        pool: Address,
        token: Address,
    ) -> Result<(f64, f64)>;
}

pub fn decimals_for_stablecoin(stablecoin: Address) -> f64 {
    match stablecoin {
        // USDT
        addr if addr
            == "0xdAC17F958D2ee523a2206206994597C13D831ec7"
                .parse()
                .unwrap() =>
        {
            1e6
        }
        // USDC
        addr if addr
            == "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
                .parse()
                .unwrap() =>
        {
            1e6
        }
        // DAI
        addr if addr
            == "0x6B175474E89094C44Da98b954EedeAC495271d0F"
                .parse()
                .unwrap() =>
        {
            1e18
        }
        _ => 1e18,
    }
}
