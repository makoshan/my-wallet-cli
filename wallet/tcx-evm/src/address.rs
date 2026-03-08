
use crate::Result;
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmAddress(pub [u8; 20]);

impl EvmAddress {
    pub fn from_public_key_bytes(pubkey: &[u8]) -> Result<Self> {
        let key = if pubkey.len() == 65 && pubkey[0] == 0x04 { &pubkey[1..] }
                  else if pubkey.len() == 64 { pubkey }
                  else { return Err(anyhow::anyhow!("bad pubkey len {}", pubkey.len())); };
        let hash = Keccak256::digest(key);
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[12..]);
        Ok(EvmAddress(addr))
    }

    pub fn to_checksum(&self) -> String {
        let hex_addr = hex::encode(self.0);
        let hash = hex::encode(Keccak256::digest(hex_addr.as_bytes()));
        let mut out = String::from("0x");
        for (i, c) in hex_addr.chars().enumerate() {
            if c.is_ascii_digit() { out.push(c); }
            else {
                let n = u8::from_str_radix(&hash[i..i+1], 16).unwrap_or(0);
                if n >= 8 { out.push(c.to_ascii_uppercase()); } else { out.push(c); }
            }
        }
        out
    }
}

impl std::fmt::Display for EvmAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_checksum())
    }
}
