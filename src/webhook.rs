/// Webhook signature verification utility.
///
/// When Hypermid sends a webhook, it includes:
///   - `X-Hypermid-Signature`: HMAC-SHA256 hex digest of the raw body
///   - `X-Hypermid-Event`: event type (e.g. "swap.completed")
///
/// Use [`verify_webhook_signature`] to validate incoming webhooks.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify a webhook signature using HMAC-SHA256.
///
/// # Arguments
/// * `body` - The raw request body as bytes
/// * `signature` - The `X-Hypermid-Signature` header value (hex-encoded HMAC)
/// * `secret` - Your webhook signing secret (from webhook creation)
///
/// # Returns
/// `true` if the signature is valid.
///
/// # Example
/// ```
/// use hypermid_sdk::webhook::verify_webhook_signature;
///
/// let body = b"{\"event\":\"swap.completed\"}";
/// let secret = "whsec_test123";
/// // In practice, signature comes from the X-Hypermid-Signature header
/// let signature = "invalid";
/// assert!(!verify_webhook_signature(body, signature, secret));
/// ```
pub fn verify_webhook_signature(body: &[u8], signature: &str, secret: &str) -> bool {
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };

    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());

    constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

/// Constant-time byte comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signature() {
        let body = b"test body";
        let secret = "test_secret";

        // Compute expected signature
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        let expected = hex::encode(mac.finalize().into_bytes());

        assert!(verify_webhook_signature(body, &expected, secret));
    }

    #[test]
    fn test_invalid_signature() {
        let body = b"test body";
        let secret = "test_secret";
        assert!(!verify_webhook_signature(body, "invalid_hex", secret));
    }

    #[test]
    fn test_wrong_body() {
        let secret = "test_secret";

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(b"original body");
        let sig = hex::encode(mac.finalize().into_bytes());

        assert!(!verify_webhook_signature(b"tampered body", &sig, secret));
    }
}
