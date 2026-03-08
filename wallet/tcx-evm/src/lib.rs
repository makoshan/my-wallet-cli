
pub mod address;
pub mod signer;
pub mod transaction;
pub mod keystore;

pub use address::EvmAddress;
pub use keystore::EvmKeystore;
pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub const CHAINS: &[&str] = &[
    "ethereum", "ethereum_sepolia", "polygon",
    "arbitrum", "optimism", "base", "avalanche", "bsc",
];
