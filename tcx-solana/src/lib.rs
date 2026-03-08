pub mod address;
pub mod signer;
pub mod transaction;

use core::result;

pub type Result<T> = result::Result<T, anyhow::Error>;

pub mod solana {
    use crate::address::SolanaAddress;

    pub const CHAINS: &[&str] = &["solana", "solana_devnet", "solana_testnet"];

    pub type Address = SolanaAddress;
    pub type TransactionInput = crate::transaction::SolanaTxInput;
    pub type TransactionOutput = crate::transaction::SolanaTxOutput;
    pub type MessageInput = crate::transaction::SolanaMessageInput;
    pub type MessageOutput = crate::transaction::SolanaMessageOutput;
}
