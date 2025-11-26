# Multi-DEX-prices

Multi-DEX-prices is a **Rust-based token price aggregator** that fetches and computes prices and approximate TVLs from multiple decentralized exchanges (DEXs). It currently supports Uniswap V2, Sushiswap, and Uniswap V3, allowing you to aggregate price data across protocols with a unified interface.

## 🔎 Why this project exists

* There are many DEX protocols — each has its own interface and quirks. This project abstracts them behind a common trait, making it easy to add support for new DEXes.
* It computes **TVL-weighted average prices** across pools — useful for getting more reliable token valuations by combining liquidity from multiple sources.
* Written in Rust: type-safe, performant, and ideal for scripting or backend use.

## ⚙️ Features

* ✅ Unified interface (`DexProtocol`) for multiple DEXes (Uniswap V2, Sushiswap, Uniswap V3)
* ✅ Price and approximate TVL calculation per pool
* ✅ Pool configuration via JSON (`config.json`) — define tokens, pool addresses, and protocols
* ✅ Aggregation logic: calculates a global “weighted average” price across all valid pools
* ✅ Easy to extend: you can add new protocols by implementing `DexProtocol`

## 📁 Project structure

```
/src
  ├── main.rs           # entry point: loads config, loops over tokens, prints prices
  ├── config.rs         # parsing and validation of config.json
  ├── dex /             # folder containing protocol-specific modules
  │     ├── mod.rs
  │     ├── uniswap_v2.rs
  │     ├── sushiswap.rs
  │     └── uniswap_v3.rs
  └── aggregator/       # aggregation logic that computes weighted average across pools
        └── mod.rs
config.json             # sample configuration file
Cargo.toml              # Rust dependency manifest
```

## 🛠️ Installation & Usage

### Prerequisites

* Rust (stable toolchain)
* A JSON config file (see “Configuration” below)
* Access to an Ethereum-compatible RPC endpoint

### Build & Run

```bash
git clone https://github.com/Frqnku/Multi-DEX-prices.git
cd Multi-DEX-prices
cargo build --release
# Copy or create config.json
cargo run --release
```

This will print the prices (and TVL where available) for all configured tokens.

## 🧮 Configuration

Use `config.json` to declare which tokens and pools you want to aggregate. Example:

```json
{
  "backend": "rpc",
  "rpc_url": "https://mainnet.infura.io/v3/<YOUR_INFURA_PROJECT_ID>",
  "tokens": [
    {
      "name": "WETH",
      "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
      "pools": [
        {
          "name": "weth_usdt",
          "address": "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852",
          "protocol": "uniswap_v2"
        },
        {
          "name": "weth_usdc",
          "address": "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8",
          "protocol": "uniswap_v3"
        }
      ]
    }
  ]
}
```

Make sure each pool includes a `protocol` from the supported list (`uniswap_v2`, `sushiswap`, `uniswap_v3`). The config validator will check this and fail early if something is invalid.

## ✅ Limitations

* TVL computation for Uniswap V3 is **approximate** (based on pool token balances), not exact - obtaining exact V3 liquidity math requires position-level data and is out of scope.
* Only ERC-20 (or ERC-20–compatible) tokens are supported. Native tokens (e.g., ETH without wrapper) are currently not handled.
* No GUI / web interface — this is a command-line / backend tool. But might add TUI soon.
* Only supports RPC for now.

## 📦 Dependencies / Tech stack

* Rust (stable)
* `ethers-rs` — for interacting with Ethereum RPC and smart contracts
* `serde` / `serde_json` — for configuration deserialization

## 📄 License

This project is released under the MIT License.

## 🙋‍♂️ Contact & Feedback

Created and maintained by Frqnku — if you encounter a bug or want to propose improvements, feel free to open an issue or send a pull request.
