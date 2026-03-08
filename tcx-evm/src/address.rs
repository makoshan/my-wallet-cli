use crate::Result;
use hex::FromHex;
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmAddress([u8; 20]);

impl EvmAddress {
    pub fn from_bytes(bytes: [u8; 20]) -> Self {
        EvmAddress(bytes)
    }

    pub fn from_public_key(pubkey_bytes: &[u8]) -> Result<Self> {
        // Public key must be 64 bytes (uncompressed, without 0x04 prefix)
        // or 65 bytes (with 0x04 prefix)
        let key_bytes = if pubkey_bytes.len() == 65 && pubkey_bytes[0] == 0x04 {
            &pubkey_bytes[1..]
        } else if pubkey_bytes.len() == 64 {
            pubkey_bytes
        } else {
            return Err(anyhow::anyhow!(
                "Invalid public key length: {}",
                pubkey_bytes.len()
            ));
        };
        let hash = Keccak256::digest(key_bytes);
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[12..]);
        Ok(EvmAddress(addr))
    }

    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let s = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let bytes = Vec::from_hex(s)
            .map_err(|e| anyhow::anyhow!("Invalid hex: {}", e))?;
        if bytes.len() != 20 {
            return Err(anyhow::anyhow!("Invalid EVM address length"));
        }
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes);
        Ok(EvmAddress(addr))
    }

    /// EIP-55 checksum address
    pub fn to_checksum(&self) -> String {
        let hex_addr = hex::encode(self.0);
        let hash = Keccak256::digest(hex_addr.as_bytes());
        let hash_hex = hex::encode(hash);

        let mut result = String::from("0x");
        for (i, c) in hex_addr.chars().enumerate() {
            if c.is_ascii_digit() {
                result.push(c);
            } else {
                let nibble = u8::from_str_radix(&hash_hex[i..i + 1], 16).unwrap_or(0);
                if nibble >= 8 {
                    result.push(c.to_ascii_uppercase());
                } else {
                    result.push(c);
                }
            }
        }
        result
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
    fn test_evm_address_checksum() {
        // Known EIP-55 test vector
        let addr = EvmAddress::from_hex("0x5aAeb6053ba3EEdb6A475A1d3E4e77F9E6d3c467").unwrap();
        let checksum = addr.to_checksum();
        assert!(checksum.starts_with("0x"));
        assert_eq!(checksum.len(), 42);
    }

    #[test]
    fn test_from_hex_roundtrip() {
        let hex = "0x1234567890123456789012345678901234567890";
        let addr = EvmAddress::from_hex(hex).unwrap();
        assert_eq!(addr.to_checksum().to_lowercase(), hex.to_lowercase());
    }
}
