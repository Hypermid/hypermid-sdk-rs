/// Execution lifecycle management -- polling helpers for deposit and LI.FI status.

use tokio::time::sleep;
use std::time::{Duration, Instant};

use crate::client::HyperMid;
use crate::error::HyperMidError;
use crate::helpers::{is_lifi_status_terminal, is_ni_status_terminal};
use crate::types::{DepositStatusParams, DepositStatusResponse, LiFiStatusParams, PollConfig, StatusParams, StatusResponse};

/// Poll Near Intents deposit/swap status until a terminal state is reached.
///
/// Terminal states: `SUCCESS`, `REFUNDED`, `FAILED`.
///
/// # Example
/// ```no_run
/// # use hypermid_sdk::client::HyperMid;
/// # use hypermid_sdk::types::{HyperMidConfig, DepositStatusParams};
/// # async fn example() -> Result<(), hypermid_sdk::error::HyperMidError> {
/// let hm = HyperMid::new(HyperMidConfig::default());
/// let status = hm.wait_for_deposit_completion(
///     &DepositStatusParams {
///         deposit_address: "0x...".to_string(),
///         deposit_memo: None,
///     },
///     None,
/// ).await?;
/// println!("Final status: {}", status.status);
/// # Ok(())
/// # }
/// ```
impl HyperMid {
    pub async fn wait_for_deposit_completion(
        &self,
        params: &DepositStatusParams,
        config: Option<PollConfig>,
    ) -> Result<DepositStatusResponse, HyperMidError> {
        let config = config.unwrap_or_default();
        let start = Instant::now();
        let mut polls: u64 = 0;

        loop {
            let status = self.get_deposit_status(params).await?;
            polls += 1;

            if is_ni_status_terminal(&status.status) {
                return Ok(status);
            }

            if start.elapsed().as_millis() as u64 >= config.max_wait_ms {
                return Err(HyperMidError::PollTimeout(format!(
                    "Deposit status polling timed out after {}ms (last status: {})",
                    config.max_wait_ms, status.status
                )));
            }
            if polls >= config.max_polls {
                return Err(HyperMidError::PollTimeout(format!(
                    "Deposit status polling exceeded {} attempts (last status: {})",
                    config.max_polls, status.status
                )));
            }

            sleep(Duration::from_millis(config.poll_interval_ms)).await;
        }
    }

    /// Poll LI.FI swap status until a terminal state is reached.
    ///
    /// Terminal states: `DONE`, `FAILED`.
    pub async fn wait_for_lifi_completion(
        &self,
        params: &LiFiStatusParams,
        config: Option<PollConfig>,
    ) -> Result<StatusResponse, HyperMidError> {
        let config = config.unwrap_or_default();
        let start = Instant::now();
        let mut polls: u64 = 0;

        let status_params = StatusParams::LiFi(params.clone());

        loop {
            let status = self.get_status(&status_params).await?;
            polls += 1;

            if let Some(ref s) = status.status {
                if is_lifi_status_terminal(s) {
                    return Ok(status);
                }
            }

            if start.elapsed().as_millis() as u64 >= config.max_wait_ms {
                return Err(HyperMidError::PollTimeout(format!(
                    "LI.FI status polling timed out after {}ms (last status: {:?})",
                    config.max_wait_ms, status.status
                )));
            }
            if polls >= config.max_polls {
                return Err(HyperMidError::PollTimeout(format!(
                    "LI.FI status polling exceeded {} attempts (last status: {:?})",
                    config.max_polls, status.status
                )));
            }

            sleep(Duration::from_millis(config.poll_interval_ms)).await;
        }
    }
}
