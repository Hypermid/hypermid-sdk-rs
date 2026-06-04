/// Error types for the Hypermid SDK.

/// All possible errors returned by SDK methods.
#[derive(Debug, thiserror::Error)]
pub enum HypermidError {
    /// API returned an error response.
    #[error("API error ({status}): [{code}] {message}")]
    Api {
        code: String,
        message: String,
        status: u16,
        details: Option<serde_json::Value>,
    },

    /// Network or connection error from reqwest.
    #[error("Request failed: {0}")]
    Network(#[from] reqwest::Error),

    /// The request exceeded the configured timeout.
    #[error("Request timed out after {0}ms")]
    Timeout(u64),

    /// Polling exceeded the maximum wait time or attempts.
    #[error("Polling timed out: {0}")]
    PollTimeout(String),

    /// JSON serialization or deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
