//! # Hypermid SDK
//!
//! Rust SDK for the [Hypermid Partner API](https://api.hypermid.io).
//!
//! Provides async access to cross-chain swap, on-ramp, partner analytics,
//! and webhook management endpoints.
//!
//! ## Quick Start
//!
//! ```ignore
//! use hypermid_sdk::client::Hypermid;
//! use hypermid_sdk::types::HypermidConfig;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let hm = Hypermid::new(HypermidConfig {
//!         api_key: Some("your-api-key".to_string()),
//!         ..Default::default()
//!     });
//!
//!     let ping = hm.ping().await?;
//!     println!("API status: {}", ping.status);
//!     Ok(())
//! }
//! ```

pub mod chains;
pub mod client;
pub mod error;
pub mod execution;
pub mod helpers;
pub mod types;
pub mod webhook;

// Re-export primary types for convenience.
pub use client::Hypermid;
pub use error::HypermidError;
pub use types::HypermidConfig;
