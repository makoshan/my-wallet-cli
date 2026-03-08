
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolanaAddress(pub [u8; 32]);

impl SolanaAddress {
    pub fn from_bytes(b: [u8; 32]) -> Self { SolanaAddress(b) }
    pub fn to_base58(&self) -> String { bs58::encode(&self.0).into_string() }
}

impl std::fmt::Display for SolanaAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}
