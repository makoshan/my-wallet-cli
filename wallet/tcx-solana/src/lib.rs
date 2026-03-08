
pub mod address;
pub mod keystore;
pub mod transaction;

pub use keystore::SolanaKeystore;
pub type Result<T> = std::result::Result<T, anyhow::Error>;
pub const CHAINS: &[&str] = &["solana", "solana_devnet", "solana_testnet"];
