use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/<API_KEY>")?;
    let provider = Arc::new(provider);

    let pool_weth_usdc: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;
    let pool_weth_usdt: Address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".parse()?;

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
    ) -> anyhow::Result<(f64, f64)> {
        let contract = UniswapV2Pair::new(pool, provider);
        let (reserve0, reserve1, _) = contract.get_reserves().call().await?;
        let token0 = contract.token_0().call().await?;
        let token1 = contract.token_1().call().await?;

        let weth_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
            .parse()
            .unwrap();

        let (reserve_weth, reserve_stable) = if token0 == weth_address {
            (reserve0, reserve1)
        } else if token1 == weth_address {
            (reserve1, reserve0)
        } else {
            panic!("Pool does not contain WETH");
        };
        let reserve_weth_f = reserve_weth as f64 / 1e18;
        let reserve_stable_f = reserve_stable as f64 / 1e6;

        let price_weth = reserve_stable_f / reserve_weth_f;
        let tvl = reserve_weth_f * price_weth + reserve_stable_f;

        Ok((price_weth, tvl))
    }

    let (price_usdc, tvl_usdc) = get_price_and_tvl(provider.clone(), pool_weth_usdc).await?;
    let (price_usdt, tvl_usdt) = get_price_and_tvl(provider.clone(), pool_weth_usdt).await?;

    let weighted_price = (price_usdc * tvl_usdc + price_usdt * tvl_usdt) / (tvl_usdc + tvl_usdt);

    println!("Prix WETH en usdc {}", price_usdc * tvl_usdc / tvl_usdc);
    println!("Prix WETH en usdt {}", price_usdt * tvl_usdt / tvl_usdt);

    println!("Prix WETH pondéré: {:.2}", weighted_price);

    Ok(())
}
