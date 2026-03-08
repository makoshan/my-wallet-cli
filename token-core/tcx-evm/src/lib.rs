pub mod address;
pub mod signer;
pub mod transaction;

use core::result;

pub type Result<T> = result::Result<T, anyhow::Error>;

pub mod evm {
    use crate::address::EvmAddress;

    pub const CHAINS: &[&str] = &[
        "ethereum",
        "ethereum_sepolia",
        "polygon",
        "arbitrum",
        "optimism",
        "base",
        "avalanche",
        "bsc",
    ];

    pub type Address = EvmAddress;
    pub type TransactionInput = crate::transaction::EvmTxInput;
    pub type TransactionOutput = crate::transaction::EvmTxOutput;
    pub type MessageInput = crate::transaction::EvmMessageInput;
    pub type MessageOutput = crate::transaction::EvmMessageOutput;
}
