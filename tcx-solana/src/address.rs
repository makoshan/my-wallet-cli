use crate::Result;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolanaAddress {
    inner: Pubkey,
}

impl SolanaAddress {
    pub fn new(pubkey: Pubkey) -> Self {
        SolanaAddress { inner: pubkey }
    }

    pub fn from_string(address_str: &str) -> Result<Self> {
        let pubkey = Pubkey::from_str(address_str)
            .map_err(|e| anyhow::anyhow!("Invalid Solana address: {}", e))?;

        Ok(SolanaAddress { inner: pubkey })
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn inner(&self) -> &Pubkey {
        &self.inner
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes().to_vec()
    }
}

impl std::fmt::Display for SolanaAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_address_from_string() {
        let addr_str = "11111111111111111111111111111111";
        let addr = SolanaAddress::from_string(addr_str).unwrap();
        assert_eq!(addr.to_string(), addr_str);
    }
}
