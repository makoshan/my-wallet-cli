pub mod address;
pub mod signer;
pub mod transaction;

use core::result;
pub type Result<T> = result::Result<T, anyhow::Error>;

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
