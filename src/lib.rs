//! # HyperMid SDK
//!
//! Rust SDK for the [HyperMid Partner API](https://api.hypermid.io).
//!
//! Provides async access to cross-chain swap, on-ramp, partner analytics,
//! and webhook management endpoints.
//!
//! ## Quick Start
//!
//! ```ignore
//! use hypermid_sdk::client::HyperMid;
//! use hypermid_sdk::types::HyperMidConfig;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let hm = HyperMid::new(HyperMidConfig {
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
pub use client::HyperMid;
pub use error::HyperMidError;
pub use types::HyperMidConfig;
