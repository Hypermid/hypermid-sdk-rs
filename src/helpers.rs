/// Type guards and utility helpers for working with Hypermid API responses.

use crate::types::{DepositMode, DepositStatusResponse, ExecuteResponse, Provider};

// ─── Type Guards ─────────────────────────────────────────────────────────

/// Check if an execute response is a LI.FI route (has transactionRequest).
pub fn is_lifi_route(response: &ExecuteResponse) -> bool {
    matches!(response, ExecuteResponse::LiFi(_))
}

/// Check if an execute response is a Near Intents route (has depositAddress).
pub fn is_near_intents_route(response: &ExecuteResponse) -> bool {
    matches!(response, ExecuteResponse::NearIntents(_))
}

/// Check if a Near Intents deposit requires manual user action (QR code / copy address).
pub fn is_manual_deposit(response: &ExecuteResponse) -> bool {
    match response {
        ExecuteResponse::NearIntents(r) => r.deposit_mode == DepositMode::Manual,
        _ => false,
    }
}

/// Check if a deposit can be done programmatically via wallet.
pub fn is_wallet_deposit(response: &ExecuteResponse) -> bool {
    match response {
        ExecuteResponse::LiFi(_) => true,
        ExecuteResponse::NearIntents(r) => r.deposit_mode == DepositMode::Wallet,
    }
}

// ─── Status Helpers ──────────────────────────────────────────────────────

/// Terminal NI deposit statuses -- polling should stop here.
const TERMINAL_NI_STATUSES: &[&str] = &["SUCCESS", "REFUNDED", "FAILED"];

/// Terminal LI.FI statuses.
const TERMINAL_LIFI_STATUSES: &[&str] = &["DONE", "FAILED"];

/// Check if a Near Intents deposit status is terminal (no more polling needed).
pub fn is_ni_status_terminal(status: &str) -> bool {
    TERMINAL_NI_STATUSES.contains(&status)
}

/// Check if a LI.FI status is terminal.
pub fn is_lifi_status_terminal(status: &str) -> bool {
    TERMINAL_LIFI_STATUSES.contains(&status)
}

/// Check if a Near Intents swap completed successfully.
pub fn is_deposit_success(response: &DepositStatusResponse) -> bool {
    response.status == "SUCCESS" && response.provider == Provider::NearIntents
}

/// Check if a Near Intents swap was refunded.
pub fn is_deposit_refunded(response: &DepositStatusResponse) -> bool {
    response.status == "REFUNDED"
}

/// Check if a Near Intents swap failed.
pub fn is_deposit_failed(response: &DepositStatusResponse) -> bool {
    response.status == "FAILED"
}
