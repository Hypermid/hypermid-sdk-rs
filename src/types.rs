/// All request and response types for the HyperMid Partner API.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ─── API Response Envelope ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiMeta {
    pub request_id: String,
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<RateLimitInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: ApiMeta,
}

// ─── Config ──────────────────────────────────────────────────────────────

/// Configuration for the HyperMid client.
#[derive(Debug, Clone)]
pub struct HyperMidConfig {
    /// API key for authenticated access (2000 req/min). Optional — anonymous = 100 req/min.
    pub api_key: Option<String>,
    /// Base URL override (default: https://api.hypermid.io).
    pub base_url: Option<String>,
    /// Request timeout in milliseconds (default: 30000).
    pub timeout_ms: Option<u64>,
}

impl Default for HyperMidConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: None,
            timeout_ms: None,
        }
    }
}

// ─── Chains ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeToken {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub id: u64,
    pub key: String,
    pub name: String,
    pub chain_type: String,
    pub native_token: NativeToken,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Additional metadata fields (logos, etc.)
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainsResponse {
    pub chains: Vec<Chain>,
}

// ─── Tokens ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub chain_id: u64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "logoURI")]
    pub logo_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "priceUSD")]
    pub price_usd: Option<String>,
    /// Additional metadata fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensResponse {
    pub tokens: HashMap<String, Vec<Token>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chains: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}

// ─── Connections ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsParams {
    pub from_chain: String,
    pub from_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<String>,
}

// ─── Gas ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPricesParams {
    pub chains: String,
}

// ─── Quote ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteParams {
    pub from_chain: String,
    pub from_token: String,
    pub from_amount: String,
    pub to_chain: String,
    pub to_token: String,
    pub from_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderPreference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderPreference {
    Recommended,
    Fastest,
    Cheapest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub quote: serde_json::Value,
    pub provider: Provider,
    pub fee_bps: u32,
    pub is_dry_quote: bool,
}

// ─── Routes ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutesParams {
    pub from_chain: String,
    pub from_token: String,
    pub from_amount: String,
    pub to_chain: String,
    pub to_token: String,
    pub from_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderPreference>,
}

// ─── Status ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiFiStatusParams {
    pub tx_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<String>,
}

/// Unified status params — either LI.FI (tx hash) or Near Intents (correlation ID).
#[derive(Debug, Clone)]
pub enum StatusParams {
    LiFi(LiFiStatusParams),
    NearIntents { correlation_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub provider: Provider,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Additional provider-specific fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

// ─── Provider ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Provider {
    #[serde(rename = "lifi")]
    LiFi,
    #[serde(rename = "near-intents")]
    NearIntents,
}

// ─── Deposit Mode ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DepositMode {
    Wallet,
    Manual,
}

// ─── Execute ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteParams {
    pub from_chain: String,
    pub from_token: String,
    pub from_amount: String,
    pub to_chain: String,
    pub to_token: String,
    pub from_address: String,
    pub to_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_mode: Option<DepositMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub to: String,
    pub data: String,
    pub value: String,
    pub from: String,
    pub chain_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    /// Additional fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiFiQuoteDetails {
    pub from_token: Token,
    pub to_token: Token,
    pub from_amount: String,
    pub to_amount: String,
    pub to_amount_min: String,
    pub estimated_time: u64,
    pub gas_costs: Vec<serde_json::Value>,
    pub fee_costs: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiFiExecuteResponse {
    pub provider: Provider,
    pub deposit_mode: DepositMode,
    pub transaction_request: TransactionRequest,
    pub quote: LiFiQuoteDetails,
    pub fee_bps: u32,
    pub instructions: LiFiInstructions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiFiInstructions {
    pub step1: String,
    pub step2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NIExecuteResponse {
    pub provider: Provider,
    pub deposit_mode: DepositMode,
    pub deposit_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_memo: Option<String>,
    pub expected_output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_estimate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    pub fee_bps: u32,
    pub instructions: NIInstructions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NIInstructions {
    pub step1: String,
    pub step2: String,
    pub step3: String,
}

/// Execute response — dispatched by provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub enum ExecuteResponse {
    #[serde(rename = "lifi")]
    LiFi(LiFiExecuteResponseInner),
    #[serde(rename = "near-intents")]
    NearIntents(NIExecuteResponseInner),
}

/// Inner LiFi response (without the provider tag, since it is used as a serde tag).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiFiExecuteResponseInner {
    pub deposit_mode: DepositMode,
    pub transaction_request: TransactionRequest,
    pub quote: LiFiQuoteDetails,
    pub fee_bps: u32,
    pub instructions: LiFiInstructions,
}

/// Inner NearIntents response (without the provider tag).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NIExecuteResponseInner {
    pub deposit_mode: DepositMode,
    pub deposit_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_memo: Option<String>,
    pub expected_output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_estimate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    pub fee_bps: u32,
    pub instructions: NIInstructions,
}

impl ExecuteResponse {
    /// Get the provider for this response.
    pub fn provider(&self) -> Provider {
        match self {
            ExecuteResponse::LiFi(_) => Provider::LiFi,
            ExecuteResponse::NearIntents(_) => Provider::NearIntents,
        }
    }

    /// Get the fee in basis points.
    pub fn fee_bps(&self) -> u32 {
        match self {
            ExecuteResponse::LiFi(r) => r.fee_bps,
            ExecuteResponse::NearIntents(r) => r.fee_bps,
        }
    }

    /// Get the deposit mode.
    pub fn deposit_mode(&self) -> &DepositMode {
        match self {
            ExecuteResponse::LiFi(r) => &r.deposit_mode,
            ExecuteResponse::NearIntents(r) => &r.deposit_mode,
        }
    }
}

// ─── Deposit (Near Intents) ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositSubmitParams {
    pub tx_hash: String,
    pub deposit_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositSubmitResponse {
    pub submitted: bool,
    pub tx_hash: String,
    pub deposit_address: String,
    pub next_step: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositStatusParams {
    pub deposit_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out_formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_chain_tx_hashes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositStatusResponse {
    pub provider: Provider,
    pub status: String,
    pub deposit_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_details: Option<SwapDetails>,
}

// ─── On-Ramp ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnrampQuoteParams {
    pub fiat_amount: serde_json::Value,
    pub fiat_currency: String,
    pub crypto_token: String,
    pub crypto_chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnrampCheckoutParams {
    pub wallet_address: String,
    pub crypto_token: String,
    pub crypto_chain: String,
    pub fiat_currency: String,
    pub fiat_amount: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnrampCheckoutResponse {
    pub redirect_url: String,
    pub order_uid: String,
    pub external_order_uid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnrampStatusResponse {
    pub status: OnrampStatus,
    pub order_uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OnrampStatus {
    Waiting,
    Processing,
    Completed,
    Failed,
    Expired,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnrampConfigResponse {
    pub chains: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnrampAssetsParams {
    pub currency: String,
    pub chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_currency: Option<String>,
}

// ─── Swap Event ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_hash: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapEventResponse {
    pub updated: bool,
    pub id: u64,
}

// ─── Partner ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub status: String,
    pub tier: String,
    pub fee_bps: u32,
    pub volume_total: f64,
    pub tx_count: u64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStat {
    pub chain: String,
    pub count: u64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStat {
    pub provider: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerStats {
    pub tx_count: u64,
    pub completed_count: u64,
    pub failed_count: u64,
    pub volume_usd: f64,
    pub fees_earned_usd: f64,
    pub avg_duration_seconds: f64,
    pub by_chain: Vec<ChainStat>,
    pub by_provider: Vec<ProviderStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub provider: String,
    pub from_chain: String,
    pub from_token: String,
    pub to_chain: String,
    pub to_token: String,
    pub amount_usd: f64,
    pub fee_usd: f64,
    pub tx_hash: String,
    pub wallet_hash: String,
    pub status: String,
    pub from_amount: String,
    pub to_amount: String,
    pub duration_seconds: u64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

// ─── Webhooks ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebhookEvent {
    #[serde(rename = "swap.completed")]
    SwapCompleted,
    #[serde(rename = "onramp.completed")]
    OnrampCompleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookParams {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub events: Vec<WebhookEvent>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCreated {
    pub id: String,
    pub url: String,
    pub events: Vec<WebhookEvent>,
    pub status: String,
    pub created_at: String,
    /// The webhook signing secret -- only returned on creation. Store securely!
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhooksListResponse {
    pub webhooks: Vec<Webhook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhookResponse {
    pub deleted: bool,
    pub id: String,
}

// ─── Ping ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingProviders {
    pub lifi: String,
    #[serde(rename = "nearIntents")]
    pub near_intents: String,
    pub rampnow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    pub status: String,
    pub version: String,
    pub uptime: f64,
    pub timestamp: u64,
    pub providers: PingProviders,
}

// ─── Polling Config ──────────────────────────────────────────────────────

/// Configuration for polling operations.
#[derive(Debug, Clone)]
pub struct PollConfig {
    /// Polling interval in milliseconds (default: 5000).
    pub poll_interval_ms: u64,
    /// Maximum time to wait for completion in milliseconds (default: 600000 = 10 min).
    pub max_wait_ms: u64,
    /// Maximum number of poll attempts (default: u64::MAX).
    pub max_polls: u64,
}

impl Default for PollConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 5_000,
            max_wait_ms: 600_000,
            max_polls: u64::MAX,
        }
    }
}

// ─── Balances ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancesParams {
    pub address: String,
    /// Restrict EVM coverage to these chain IDs (sent as a comma-separated
    /// `chainIds` query param).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_ids: Option<Vec<u64>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub chain_id: u64,
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u32,
    pub balance: String,
    pub price_usd: f64,
    pub balance_usd: f64,
    pub logo_uri: String,
    #[serde(default)]
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceChainMeta {
    pub ok: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    pub duration_ms: u64,
    #[serde(default)]
    pub stale: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancesResponse {
    pub address: String,
    pub total_balance_usd: String,
    pub balances: HashMap<String, Vec<TokenBalance>>,
    #[serde(default)]
    pub chain_meta: Option<HashMap<String, BalanceChainMeta>>,
    #[serde(default)]
    pub cached_at: Option<String>,
    #[serde(default)]
    pub cache_hit: Option<bool>,
}

// ─── Inbound receiver (SuperSwap V2) ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundReceiverParams {
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub output_token: String,
    pub destination_domain: u64,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundReceiverResponse {
    pub registered: bool,
    pub record_id: String,
    pub usdc_amount: String,
    pub status: String,
}
