use crate::Result;
use bs58;
use std::str::FromStr;
use tcx_constants::CoinInfo;
use tcx_keystore::Address;
use tcx_primitive::TypedPublicKey;

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

impl FromStr for SolanaAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_base58(s)
    }
}

impl Address for SolanaAddress {
    fn from_public_key(public_key: &TypedPublicKey, _coin: &CoinInfo) -> Result<Self> {
        SolanaAddress::from_ed25519_pubkey(&public_key.to_bytes())
    }

    fn is_valid(address: &str, _coin: &CoinInfo) -> bool {
        SolanaAddress::from_str(address).is_ok()
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
