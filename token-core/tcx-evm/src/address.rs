use crate::Result;
use alloy_primitives::Address as AlloyAddress;
use hex::{FromHex, ToHex};
use std::str::FromStr;
use tcx_common::FromHex as TcxFromHex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmAddress {
    inner: AlloyAddress,
}

impl EvmAddress {
    pub fn new(address: AlloyAddress) -> Self {
        EvmAddress { inner: address }
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex_str = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        let bytes = Vec::from_hex(hex_str)?;
        if bytes.len() != 20 {
            return Err(anyhow::anyhow!("Invalid EVM address length"));
        }

        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes);
        Ok(EvmAddress {
            inner: AlloyAddress::from(addr),
        })
    }

    pub fn to_checksum(&self) -> String {
        // EIP-55 checksum address
        let addr_hex = self.inner.to_string().to_lowercase();
        let addr_hex = if addr_hex.starts_with("0x") {
            &addr_hex[2..]
        } else {
            &addr_hex
        };

        let hash = sha3::Keccak256::digest(addr_hex.as_bytes());
        let hash_hex = hash.to_hex::<String>();

        let mut result = String::from("0x");
        for (i, c) in addr_hex.chars().enumerate() {
            if let Some(h) = hash_hex.chars().nth(i) {
                if c.is_ascii_digit() {
                    result.push(c);
                } else if h.to_digit(16).unwrap() >= 8 {
                    result.push(c.to_ascii_uppercase());
                } else {
                    result.push(c);
                }
            }
        }
        result
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn inner(&self) -> &AlloyAddress {
        &self.inner
    }
}

impl std::fmt::Display for EvmAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_checksum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_address_from_hex() {
        let addr_str = "0x1234567890123456789012345678901234567890";
        let addr = EvmAddress::from_hex(addr_str).unwrap();
        assert_eq!(addr.to_string().to_lowercase(), addr_str.to_lowercase());
    }

    #[test]
    fn test_evm_address_checksum() {
        let addr_str = "0x5aAeb6053ba3EEdb6A475A1d3E4e77F9E6d3c467";
        let addr = EvmAddress::from_hex(addr_str).unwrap();
        let checksum = addr.to_checksum();
        assert!(checksum.starts_with("0x"));
    }
}
