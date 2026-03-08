use crate::Result;
use bs58;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolanaAddress(pub [u8; 32]);

impl SolanaAddress {
    pub fn from_ed25519_pubkey(pubkey: &[u8]) -> Result<Self> {
        if pubkey.len() != 32 {
            return Err(anyhow::anyhow!("Solana public key must be 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(pubkey);
        Ok(SolanaAddress(arr))
    }

    pub fn from_base58(s: &str) -> Result<Self> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|e| anyhow::anyhow!("Invalid base58: {}", e))?;
        if bytes.len() != 32 {
            return Err(anyhow::anyhow!("Invalid Solana address length"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(SolanaAddress(arr))
    }

    pub fn to_base58(&self) -> String {
        bs58::encode(&self.0).into_string()
    }
}

impl std::fmt::Display for SolanaAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_address_roundtrip() {
        let bytes = [1u8; 32];
        let addr = SolanaAddress(bytes);
        let s = addr.to_base58();
        let addr2 = SolanaAddress::from_base58(&s).unwrap();
        assert_eq!(addr, addr2);
    }
}
