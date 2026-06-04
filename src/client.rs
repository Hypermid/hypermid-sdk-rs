/// Hypermid API client.

use std::time::Duration;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::error::HypermidError;
use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://api.hypermid.io";
const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// The main Hypermid SDK client.
#[derive(Debug, Clone)]
pub struct Hypermid {
    base_url: String,
    api_key: Option<String>,
    timeout_ms: u64,
    client: Client,
}

impl Hypermid {
    /// Create a new Hypermid client with the given configuration.
    pub fn new(config: HypermidConfig) -> Self {
        let timeout_ms = config.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: config
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
                .trim_end_matches('/')
                .to_string(),
            api_key: config.api_key,
            timeout_ms,
            client,
        }
    }

    /// Create a new client with default configuration (anonymous, 100 req/min).
    pub fn anonymous() -> Self {
        Self::new(HypermidConfig::default())
    }

    // ─── Internal helpers ────────────────────────────────────────────────

    async fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        params: Option<&[(&str, &str)]>,
        body: Option<serde_json::Value>,
    ) -> Result<T, HypermidError> {
        let url = format!("{}/v1{}", self.base_url, path);

        let mut req = self.client.request(method.clone(), &url);

        if let Some(key) = &self.api_key {
            req = req.header("X-API-Key", key);
        }

        if let Some(params) = params {
            let filtered: Vec<(&str, &str)> = params
                .iter()
                .filter(|(_, v)| !v.is_empty())
                .copied()
                .collect();
            if !filtered.is_empty() {
                req = req.query(&filtered);
            }
        }

        if let Some(body) = body {
            req = req.header("Content-Type", "application/json").json(&body);
        }

        let res = req.send().await.map_err(|e| {
            if e.is_timeout() {
                HypermidError::Timeout(self.timeout_ms)
            } else {
                HypermidError::Network(e)
            }
        })?;

        let status = res.status().as_u16();
        let text = res.text().await.map_err(HypermidError::Network)?;

        let envelope: ApiResponse<serde_json::Value> =
            serde_json::from_str(&text).map_err(HypermidError::Json)?;

        if let Some(err) = envelope.error {
            return Err(HypermidError::Api {
                code: err.code,
                message: err.message,
                status,
                details: err.details,
            });
        }

        match envelope.data {
            Some(data) => serde_json::from_value(data).map_err(HypermidError::Json),
            None => {
                // Try deserializing from unit / empty for responses with no data
                serde_json::from_value(serde_json::Value::Null).map_err(HypermidError::Json)
            }
        }
    }

    async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<T, HypermidError> {
        self.request(reqwest::Method::GET, path, params, None).await
    }

    async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T, HypermidError> {
        self.request(reqwest::Method::POST, path, None, Some(body))
            .await
    }

    async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, HypermidError> {
        self.request(reqwest::Method::DELETE, path, None, None)
            .await
    }

    // ─── Core Swap Endpoints ─────────────────────────────────────────────

    /// Get all supported chains (LI.FI + Near Intents). Cached server-side for 1 hour.
    pub async fn get_chains(&self) -> Result<ChainsResponse, HypermidError> {
        self.get("/chains", None).await
    }

    /// Get available tokens, optionally filtered by chains and keywords.
    /// Cached server-side for 5 minutes.
    pub async fn get_tokens(
        &self,
        params: Option<&TokensParams>,
    ) -> Result<TokensResponse, HypermidError> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        let chains_str;
        let keywords_str;

        if let Some(p) = params {
            if let Some(c) = &p.chains {
                chains_str = c.clone();
                query.push(("chains", &chains_str));
            }
            if let Some(k) = &p.keywords {
                keywords_str = k.clone();
                query.push(("keywords", &keywords_str));
            }
        }

        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.get("/tokens", q).await
    }

    /// Get available connections (which token pairs can be swapped).
    pub async fn get_connections(
        &self,
        params: &ConnectionsParams,
    ) -> Result<serde_json::Value, HypermidError> {
        let mut query = vec![
            ("fromChain", params.from_chain.as_str()),
            ("fromToken", params.from_token.as_str()),
        ];
        let to_chain;
        if let Some(tc) = &params.to_chain {
            to_chain = tc.clone();
            query.push(("toChain", &to_chain));
        }
        self.get("/connections", Some(&query)).await
    }

    /// Get available bridge/swap tools. Cached server-side for 1 hour.
    pub async fn get_tools(&self) -> Result<serde_json::Value, HypermidError> {
        self.get("/tools", None).await
    }

    /// Get gas prices for specified chains.
    pub async fn get_gas_prices(
        &self,
        params: &GasPricesParams,
    ) -> Result<serde_json::Value, HypermidError> {
        self.get("/gas-prices", Some(&[("chains", params.chains.as_str())]))
            .await
    }

    /// Get the best swap quote for a token pair.
    pub async fn get_quote(&self, params: &QuoteParams) -> Result<QuoteResponse, HypermidError> {
        let mut query = vec![
            ("fromChain", params.from_chain.clone()),
            ("fromToken", params.from_token.clone()),
            ("fromAmount", params.from_amount.clone()),
            ("toChain", params.to_chain.clone()),
            ("toToken", params.to_token.clone()),
            ("fromAddress", params.from_address.clone()),
        ];

        if let Some(ref ta) = params.to_address {
            query.push(("toAddress", ta.clone()));
        }
        if let Some(ref s) = params.slippage {
            query.push(("slippage", s.clone()));
        }
        if let Some(ref o) = params.order {
            let s = serde_json::to_string(o).unwrap_or_default();
            query.push(("order", s.trim_matches('"').to_string()));
        }

        let refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get("/quote", Some(&refs)).await
    }

    /// Get available routes for a token pair (multi-route comparison).
    pub async fn get_routes(
        &self,
        params: &RoutesParams,
    ) -> Result<serde_json::Value, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/routes", body).await
    }

    /// Check the status of a cross-chain swap.
    pub async fn get_status(
        &self,
        params: &StatusParams,
    ) -> Result<StatusResponse, HypermidError> {
        match params {
            StatusParams::NearIntents { correlation_id } => {
                self.get(
                    "/status",
                    Some(&[
                        ("provider", "near-intents"),
                        ("correlationId", correlation_id),
                    ]),
                )
                .await
            }
            StatusParams::LiFi(p) => {
                let mut query = vec![("txHash", p.tx_hash.as_str())];
                let bridge;
                let from_chain;
                let to_chain;

                if let Some(b) = &p.bridge {
                    bridge = b.clone();
                    query.push(("bridge", &bridge));
                }
                if let Some(fc) = &p.from_chain {
                    from_chain = fc.clone();
                    query.push(("fromChain", &from_chain));
                }
                if let Some(tc) = &p.to_chain {
                    to_chain = tc.clone();
                    query.push(("toChain", &to_chain));
                }
                self.get("/status", Some(&query)).await
            }
        }
    }

    // ─── Execute ─────────────────────────────────────────────────────────

    /// Get full transaction data for execution.
    ///
    /// **LI.FI routes**: Returns `ExecuteResponse::LiFi` with `transactionRequest` -- sign and broadcast.
    ///
    /// **Near Intents routes**: Returns `ExecuteResponse::NearIntents` with `depositAddress` -- send tokens.
    pub async fn execute(
        &self,
        params: &ExecuteParams,
    ) -> Result<ExecuteResponse, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/execute", body).await
    }

    /// Submit a deposit transaction hash after sending tokens to a Near Intents deposit address.
    /// Only needed for `deposit_mode: Wallet`. Manual deposits are auto-detected.
    pub async fn submit_deposit(
        &self,
        params: &DepositSubmitParams,
    ) -> Result<DepositSubmitResponse, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/execute/deposit/submit", body).await
    }

    /// Check the status of a Near Intents deposit/swap.
    pub async fn get_deposit_status(
        &self,
        params: &DepositStatusParams,
    ) -> Result<DepositStatusResponse, HypermidError> {
        let mut query = vec![("depositAddress", params.deposit_address.as_str())];
        let memo;
        if let Some(m) = &params.deposit_memo {
            memo = m.clone();
            query.push(("depositMemo", &memo));
        }
        self.get("/execute/deposit/status", Some(&query)).await
    }

    // ─── On-Ramp ─────────────────────────────────────────────────────────

    /// Get a fiat to crypto price quote.
    pub async fn get_onramp_quote(
        &self,
        params: &OnrampQuoteParams,
    ) -> Result<serde_json::Value, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/onramp/quote", body).await
    }

    /// Create a fiat to crypto purchase session. Returns a redirect URL to the payment page.
    pub async fn create_onramp_checkout(
        &self,
        params: &OnrampCheckoutParams,
    ) -> Result<OnrampCheckoutResponse, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/onramp/checkout", body).await
    }

    /// Check on-ramp order status.
    pub async fn get_onramp_status(
        &self,
        order_uid: &str,
    ) -> Result<OnrampStatusResponse, HypermidError> {
        self.get("/onramp/status", Some(&[("orderUid", order_uid)]))
            .await
    }

    /// Get supported chains and tokens for on-ramp. Cached server-side for 5 minutes.
    pub async fn get_onramp_config(&self) -> Result<OnrampConfigResponse, HypermidError> {
        self.get("/onramp/config", None).await
    }

    /// Get asset config (min/max amounts, precision, payment methods).
    pub async fn get_onramp_assets(
        &self,
        params: &OnrampAssetsParams,
    ) -> Result<serde_json::Value, HypermidError> {
        let mut query = vec![
            ("currency", params.currency.as_str()),
            ("chain", params.chain.as_str()),
        ];
        let order_currency;
        if let Some(oc) = &params.order_currency {
            order_currency = oc.clone();
            query.push(("orderCurrency", &order_currency));
        }
        self.get("/onramp/assets", Some(&query)).await
    }

    // ─── Balances ────────────────────────────────────────────────────────

    /// Multi-ecosystem token balances + total USD value for an address. The
    /// backend auto-detects the address ecosystem (EVM / Sui / Tron / NEAR /
    /// Solana / Bitcoin); pass `chain_ids` to restrict EVM coverage.
    pub async fn get_balances(
        &self,
        params: &BalancesParams,
    ) -> Result<BalancesResponse, HypermidError> {
        let mut query: Vec<(&str, &str)> = vec![("address", params.address.as_str())];
        let chain_ids_str;
        if let Some(ids) = &params.chain_ids {
            if !ids.is_empty() {
                chain_ids_str = ids
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                query.push(("chainIds", &chain_ids_str));
            }
        }
        self.get("/balances", Some(query.as_slice())).await
    }

    // ─── Inbound receiver (SuperSwap V2) ─────────────────────────────────

    /// Register a SuperSwap V2 inbound deposit so the backend executes the
    /// PulseChain-side output. The deposit must already be on-chain, and an
    /// EIP-712 signature over the registration is required.
    pub async fn register_inbound_receiver(
        &self,
        params: &InboundReceiverParams,
    ) -> Result<InboundReceiverResponse, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/inbound-receiver/register", body).await
    }

    // ─── Swap Event ──────────────────────────────────────────────────────

    /// Record a swap event for analytics. `partner_id` is automatically attributed from your API key.
    pub async fn record_swap_event(
        &self,
        params: &SwapEventParams,
    ) -> Result<SwapEventResponse, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/swap-event", body).await
    }

    // ─── Partner (requires API key) ──────────────────────────────────────

    /// Get your partner info (requires API key).
    pub async fn get_partner_info(&self) -> Result<PartnerInfo, HypermidError> {
        self.get("/partner/me", None).await
    }

    /// Get volume, fee, and performance stats (requires API key).
    pub async fn get_partner_stats(
        &self,
        params: Option<&PartnerStatsParams>,
    ) -> Result<PartnerStats, HypermidError> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        let from_str;
        let to_str;

        if let Some(p) = params {
            if let Some(f) = &p.from {
                from_str = f.clone();
                query.push(("from", &from_str));
            }
            if let Some(t) = &p.to {
                to_str = t.clone();
                query.push(("to", &to_str));
            }
        }

        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.get("/partner/stats", q).await
    }

    /// Get paginated transaction history (requires API key).
    pub async fn get_partner_transactions(
        &self,
        params: Option<&PaginationParams>,
    ) -> Result<PaginatedResponse<Transaction>, HypermidError> {
        let mut query: Vec<(&str, String)> = Vec::new();

        if let Some(p) = params {
            if let Some(page) = p.page {
                query.push(("page", page.to_string()));
            }
            if let Some(limit) = p.limit {
                query.push(("limit", limit.to_string()));
            }
        }

        let refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q = if refs.is_empty() {
            None
        } else {
            Some(refs.as_slice())
        };
        self.get("/partner/transactions", q).await
    }

    // ─── Webhooks (requires API key) ─────────────────────────────────────

    /// Register a webhook endpoint. Returns the signing secret -- store it securely!
    /// The secret is only shown once, on creation.
    pub async fn create_webhook(
        &self,
        params: &CreateWebhookParams,
    ) -> Result<WebhookCreated, HypermidError> {
        let body = serde_json::to_value(params).map_err(HypermidError::Json)?;
        self.post("/partner/webhooks", body).await
    }

    /// List all registered webhooks (requires API key).
    pub async fn list_webhooks(&self) -> Result<WebhooksListResponse, HypermidError> {
        self.get("/partner/webhooks", None).await
    }

    /// Delete a webhook by ID (requires API key).
    pub async fn delete_webhook(
        &self,
        webhook_id: &str,
    ) -> Result<DeleteWebhookResponse, HypermidError> {
        self.delete(&format!("/partner/webhooks/{}", webhook_id))
            .await
    }

    // ─── Health Check ────────────────────────────────────────────────────

    /// Simple health check. Returns API status, version, uptime, and provider statuses.
    pub async fn ping(&self) -> Result<PingResponse, HypermidError> {
        self.get("/ping", None).await
    }
}
