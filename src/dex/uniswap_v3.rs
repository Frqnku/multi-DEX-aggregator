use crate::dex::DexProtocol;

use anyhow::Result;
use ethers::{
    abi::Address,
    contract::abigen,
    providers::{Http, Provider},
    types::U256,
};
use std::sync::Arc;

pub struct UniswapV3;

abigen!(
    ERC20,
    r#"[
        function decimals() external view returns (uint8)
        function balanceOf(address owner) external view returns (uint256)
    ]"#
);

abigen!(
    UniswapV3Pool,
    r#"[
        function slot0() external view returns (uint160 sqrtPriceX96,int24 tick,uint16 observationIndex,uint16 observationCardinality,uint16 observationCardinalityNext,uint8 feeProtocol,bool unlocked)
        function token0() external view returns (address)
        function token1() external view returns (address)
        function fee() external view returns (uint24)
    ]"#
);

#[async_trait::async_trait]
impl DexProtocol for UniswapV3 {
    async fn get_price_and_tvl(
        &self,
        provider: Arc<Provider<Http>>,
        pool: Address,
        token: Address,
    ) -> Result<(f64, f64)> {
        let contract = UniswapV3Pool::new(pool, provider.clone());

        // Read pool tokens
        let token0 = contract.token_0().call().await?;
        let token1 = contract.token_1().call().await?;

        if token != token0 && token != token1 {
            panic!("Token not found in Uniswap V3 pool");
        }

        // Load decimals
        let erc0 = ERC20::new(token0, provider.clone());
        let erc1 = ERC20::new(token1, provider.clone());

        let dec0 = erc0.decimals().call().await?;
        let dec1 = erc1.decimals().call().await?;

        // slot0 gives sqrtPriceX96
        let (sqrt_price_x96, _, _, _, _, _, _) = contract.slot_0().call().await?;

        // Convert sqrtPriceX96 → price ratio (token1/token0)
        let sqrt_f = sqrt_price_x96.as_u128() as f64;
        let price_1_over_0 = (sqrt_f * sqrt_f) / (2_f64.powi(192));

        // Decimal correction
        let decimal_factor = 10f64.powi(dec0 as i32 - dec1 as i32);
        let adjusted_price = price_1_over_0 * decimal_factor;

        // Final price depending on which token is requested
        let final_price = if token == token0 {
            adjusted_price
        } else {
            1.0 / adjusted_price
        };

        // --- TVL approximation via pool balances ---
        let balance0: U256 = erc0.balance_of(pool).call().await?;
        let balance1: U256 = erc1.balance_of(pool).call().await?;

        let balance0_f = balance0.as_u128() as f64 / 10f64.powi(dec0 as i32);
        let balance1_f = balance1.as_u128() as f64 / 10f64.powi(dec1 as i32);

        let tvl = if token == token0 {
            balance0_f * final_price + balance1_f
        } else {
            balance1_f * final_price + balance0_f
        };

        Ok((final_price, tvl))
    }
}
