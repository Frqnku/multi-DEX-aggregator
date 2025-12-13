use ethers::prelude::*;
use std::sync::Arc;
use std::time::Instant;

use crate::config::Config;
mod aggregator;
mod config;
mod dex;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    ui::print_banner();

    // Load configuration
    ui::print_section_header("Configuration");
    let cfg = match Config::from_file("config.json") {
        Ok(c) => {
            ui::print_success("Config loaded from config.json");
            ui::print_info(&format!(
                "RPC: {}",
                c.rpc_url.as_ref().unwrap_or(&"N/A".to_string())
            ));
            ui::print_info(&format!("Tokens to process: {}", c.tokens.len()));
            c
        }
        Err(e) => {
            ui::print_error(&format!("Failed to load config: {}", e));
            return Err(e);
        }
    };

    // Initialize provider
    ui::print_section_header("Connecting to RPC");
    let provider = match Provider::<Http>::try_from(cfg.rpc_url.unwrap()) {
        Ok(p) => {
            ui::print_success("Connected to RPC provider");
            Arc::new(p)
        }
        Err(e) => {
            ui::print_error(&format!("Failed to connect: {}", e));
            return Err(e.into());
        }
    };

    // Process tokens
    ui::print_section_header("Fetching Token Prices");
    let pb = ui::create_progress_bar(cfg.tokens.len() as u64, "Processing tokens");

    let mut results = Vec::new();

    for token in &cfg.tokens {
        match aggregator::compute_token_price(provider.clone(), token).await {
            Ok(price) => {
                pb.suspend(|| {
                    ui::print_token_result(&token.name, price, token.pools.len());
                });
                results.push((token.name.as_str(), price));
            }
            Err(e) => {
                pb.suspend(|| {
                    ui::print_error(&format!("Failed to get price for {}: {}", token.name, e));
                });
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("✓ All tokens processed successfully");
    println!();

    // Display results table
    ui::print_section_header("Price Summary");
    let results_refs: Vec<(&str, f64)> = results.iter().map(|(k, v)| (*k, *v)).collect();
    ui::print_price_table(results_refs);

    let duration = start.elapsed();
    ui::print_footer(duration);

    Ok(())
}
