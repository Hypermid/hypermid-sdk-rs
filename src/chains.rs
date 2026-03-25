/// Chain ID constants for all supported chains.
///
/// # Example
/// ```
/// use hypermid_sdk::chains::ChainId;
///
/// let from_chain = ChainId::ETHEREUM;
/// let to_chain = ChainId::ARBITRUM;
/// ```

const NI_BASE: u64 = 900_000_000;

/// Chain ID constants.
pub struct ChainId;

impl ChainId {
    // ─── EVM Chains ──────────────────────────────────────────────
    pub const ETHEREUM: u64 = 1;
    pub const OPTIMISM: u64 = 10;
    pub const BSC: u64 = 56;
    pub const GNOSIS: u64 = 100;
    pub const POLYGON: u64 = 137;
    pub const X_LAYER: u64 = 196;
    pub const ARBITRUM: u64 = 42161;
    pub const AVALANCHE: u64 = 43114;
    pub const BASE: u64 = 8453;
    pub const PLASMA: u64 = 1012;
    pub const BERACHAIN: u64 = 80094;
    pub const MONAD: u64 = 10143;

    // ─── Non-EVM (LI.FI supported) ──────────────────────────────
    pub const SOLANA: u64 = 1151111081099710;
    pub const BITCOIN: u64 = 20000000000001;
    pub const SUI: u64 = 9270000000000000;

    // ─── Near Intents-only chains ────────────────────────────────
    pub const NEAR: u64 = NI_BASE + 1;
    pub const TON: u64 = NI_BASE + 2;
    pub const TRON: u64 = NI_BASE + 3;
    pub const XRP: u64 = NI_BASE + 4;
    pub const DOGECOIN: u64 = NI_BASE + 5;
    pub const LITECOIN: u64 = NI_BASE + 6;
    pub const BITCOIN_CASH: u64 = NI_BASE + 7;
    pub const STELLAR: u64 = NI_BASE + 8;
    pub const CARDANO: u64 = NI_BASE + 9;
    pub const APTOS: u64 = NI_BASE + 10;
    pub const STARKNET: u64 = NI_BASE + 11;
    pub const DASH: u64 = NI_BASE + 12;
    pub const ZCASH: u64 = NI_BASE + 13;
    pub const ALEO: u64 = NI_BASE + 14;
    pub const ADI: u64 = NI_BASE + 15;
}

/// Check if a chain ID belongs to a Near Intents-only chain.
pub fn is_near_intents_chain(chain_id: u64) -> bool {
    chain_id >= NI_BASE && chain_id < NI_BASE + 1000
}

/// Check if a chain supports wallet-connected deposit mode.
///
/// Chains with wallet connectors: EVM, Solana, Bitcoin, Sui, TON, Tron.
/// Other NI chains (NEAR, XRP, DOGE, etc.) require manual deposit.
pub fn supports_wallet_deposit(chain_id: u64) -> bool {
    if chain_id > 0 && chain_id < NI_BASE {
        return true; // EVM
    }
    matches!(
        chain_id,
        ChainId::SOLANA | ChainId::BITCOIN | ChainId::SUI | ChainId::TON | ChainId::TRON
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_near_intents_chain() {
        assert!(!is_near_intents_chain(ChainId::ETHEREUM));
        assert!(!is_near_intents_chain(ChainId::SOLANA));
        assert!(is_near_intents_chain(ChainId::NEAR));
        assert!(is_near_intents_chain(ChainId::TON));
        assert!(is_near_intents_chain(ChainId::TRON));
        assert!(is_near_intents_chain(ChainId::ADI));
    }

    #[test]
    fn test_supports_wallet_deposit() {
        assert!(supports_wallet_deposit(ChainId::ETHEREUM));
        assert!(supports_wallet_deposit(ChainId::ARBITRUM));
        assert!(supports_wallet_deposit(ChainId::SOLANA));
        assert!(supports_wallet_deposit(ChainId::TON));
        assert!(supports_wallet_deposit(ChainId::TRON));
        assert!(!supports_wallet_deposit(ChainId::NEAR));
        assert!(!supports_wallet_deposit(ChainId::XRP));
        assert!(!supports_wallet_deposit(ChainId::DOGECOIN));
    }
}
