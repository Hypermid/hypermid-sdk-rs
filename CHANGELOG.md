# Changelog

## 0.3.0

### Breaking

- **`Provider` enum**: added `Provider::SuperSwap` (`"superswap"`). Code that
  matches `Provider` exhaustively must add an arm. Before this release, a
  `provider: "superswap"` response from `/quote`, `/execute`, or `/status`
  **failed to deserialize at runtime** — this release fixes that.
- **`ExecuteResponse` enum**: added the `ExecuteResponse::SuperSwap(..)` variant
  (tagged `"superswap"`) wrapping the new `SuperSwapExecuteResponseInner`. Code
  that matches `ExecuteResponse` exhaustively must add an arm.

### Added

- `SuperSwapExecuteResponseInner` — SuperSwap V2 executable envelope:
  `transaction_request`, `source`, `approval_address`, `estimated_output`,
  `min_output`, `fee_bps`, plus a flattened `extra` map so unknown V2 fields
  never break deserialization.
- `helpers::is_superswap_route(&ExecuteResponse) -> bool`.
- `helpers::is_superswap_status_terminal(&str) -> bool` (terminal: `DONE`, `FAILED`).
- `DepositMode` now derives `Default` (`Wallet`).
