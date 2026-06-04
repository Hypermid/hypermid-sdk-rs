# hypermid-sdk

Rust SDK for the [Hypermid](https://hypermid.io) Partner API — swap,
bridge, and on-ramp across 90+ chains (EVM, Solana, Bitcoin, Sui, NEAR,
Tron, TON, XRP, Doge).

```toml
[dependencies]
hypermid-sdk = "0.2"
```

## Quick start

No API key required. The SDK works anonymously out of the box at the
default fee tier — pass an API key only if you're a partner with
custom fee terms.

```rust
use hypermid_sdk::{Hypermid, HypermidConfig};
use hypermid_sdk::types::QuoteParams;

#[tokio::main]
async fn main() -> Result<(), hypermid_sdk::error::HypermidError> {
    // Anonymous — works immediately, no signup
    let hm = Hypermid::new(HypermidConfig::default());

    // Or, partner with custom fees:
    // let hm = Hypermid::new(HypermidConfig {
    //     api_key: Some(std::env::var("HYPERMID_API_KEY")?),
    //     ..Default::default()
    // });

    let chains = hm.get_chains().await?;
    println!("Supported chains: {}", chains.chains.len());

    Ok(())
}
```

## Features

- `get_quote` / `execute` / `get_status` — the swap pipeline
- `get_chains` / `get_tokens` — supported chains and tokens
- `get_balances` — multi-ecosystem wallet balances + USD totals
- `register_inbound_receiver` — SuperSwap V2 inbound deposits
- On-ramp helpers — `get_onramp_quote`, `get_onramp_checkout`, `get_onramp_status`
- `verify_webhook_signature` — HMAC webhook signature verification

## Authentication

The API is open by default — every endpoint works without
authentication, so you can integrate, test, and ship without a signup.

An **API key is only needed if you're a partner** with negotiated terms
(custom fee splits, fee discounts, volume tiers, higher rate limits,
webhook events scoped to your traffic). When set, the SDK sends it as
the `X-API-Key` header.

Apply for a partner account at [partner.hypermid.io](https://partner.hypermid.io).

## Documentation

Full reference: <https://docs.hypermid.io>

## License

MIT
