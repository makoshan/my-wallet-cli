use crate::Result;
use bitcoin::address::ParseError;
use bitcoin::{Address as BitcoinAddressStd, PublicKey};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAddress {
    inner: BitcoinAddressStd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    Legacy,
    SegWit,
    Taproot,
}

impl BitcoinAddress {
    pub fn new(address: BitcoinAddressStd) -> Self {
        BitcoinAddress { inner: address }
    }

    pub fn from_string(address_str: &str, network: bitcoin::Network) -> Result<Self> {
        let addr = BitcoinAddressStd::from_str(address_str)
            .map_err(|e| anyhow::anyhow!("Invalid Bitcoin address: {}", e))?
            .assume_checked();

        Ok(BitcoinAddress { inner: addr })
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn address_type(&self) -> AddressType {
        match self.inner.address_type() {
            Some(bitcoin::address::AddressType::P2pkh) => AddressType::Legacy,
            Some(bitcoin::address::AddressType::P2sh) => AddressType::Legacy,
            Some(bitcoin::address::AddressType::P2wpkh) => AddressType::SegWit,
            Some(bitcoin::address::AddressType::P2wsh) => AddressType::SegWit,
            Some(bitcoin::address::AddressType::P2tr) => AddressType::Taproot,
            None => AddressType::Legacy,
        }
    }

    pub fn inner(&self) -> &BitcoinAddressStd {
        &self.inner
    }
}

impl std::fmt::Display for BitcoinAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoin_address_from_string() {
        let addr_str = "1A1z7agoat2LWQLZLV37ZLX4My6ps6nFX";
        let addr = BitcoinAddress::from_string(addr_str, bitcoin::Network::Bitcoin).unwrap();
        assert_eq!(addr.to_string(), addr_str);
    }
}
