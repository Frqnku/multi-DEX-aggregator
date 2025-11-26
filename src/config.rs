use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashSet, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub backend: String,
    pub rpc_url: Option<String>,
    pub tokens: Vec<TokenConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TokenConfig {
    pub name: String,
    pub token: String,
    pub pools: Vec<PoolConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PoolConfig {
    pub name: String,
    pub address: String,
    pub protocol: String,
}

const SUPPORTED_PROTOCOLS: [&str; 3] = ["uniswap_v2", "sushiswap", "uniswap_v3"];

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Unable to read config file {:?}", path.as_ref()))?;

        let config: Config =
            serde_json::from_str(&content).context("Failed to parse config.json")?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        match self.backend.as_str() {
            "rpc" => {
                if self.rpc_url.is_none() {
                    anyhow::bail!("backend=rpc but no 'rpc_url' provided");
                }
            }
            "substreams" => anyhow::bail!("substreams backend not yet supported"),
            other => anyhow::bail!("Unknown backend '{other}'"),
        }

        if self.tokens.is_empty() {
            anyhow::bail!("'tokens' cannot be empty");
        }

        let mut duplicate_tokens: HashSet<&String> = std::collections::HashSet::new();

        for token in &self.tokens {
            if duplicate_tokens.contains(&&token.token) {
                anyhow::bail!("Duplicate token found: '{}'", token.token);
            } else {
                duplicate_tokens.insert(&token.token);
            }

            if token.pools.is_empty() {
                anyhow::bail!("token '{}' must contain at least one pool", token.name);
            }

            for pool in &token.pools {
                if pool.address.len() != 42 {
                    anyhow::bail!(
                        "Invalid pool address '{}' for pair '{}'",
                        pool.address,
                        pool.name
                    );
                }

                if pool.protocol.is_empty() {
                    anyhow::bail!(
                        "Protocol not specified for pool '{}' of token '{}'",
                        pool.name,
                        token.name
                    );
                }

                if !SUPPORTED_PROTOCOLS.contains(&pool.protocol.as_str()) {
                    anyhow::bail!(
                        "Unsupported protocol '{}' for pool '{}' of token '{}'",
                        pool.protocol,
                        pool.name,
                        token.name
                    );
                }
            }
        }

        Ok(())
    }
}
