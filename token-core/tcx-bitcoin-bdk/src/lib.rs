pub mod address;
pub mod signer;
pub mod transaction;

use core::result;

pub type Result<T> = result::Result<T, anyhow::Error>;

pub mod bitcoin {
    use crate::address::BitcoinAddress;

    pub const CHAINS: &[&str] = &["bitcoin", "bitcoin_testnet", "bitcoin_signet"];

    pub type Address = BitcoinAddress;
    pub type TransactionInput = crate::transaction::BitcoinTxInput;
    pub type TransactionOutput = crate::transaction::BitcoinTxOutput;
    pub type MessageInput = crate::transaction::BitcoinMessageInput;
    pub type MessageOutput = crate::transaction::BitcoinMessageOutput;
}
