use crate::Result;
use bitcoin::util::address::Address;
use bitcoin::Network;
use std::str::FromStr;

pub fn network_from_str(network: &str) -> Network {
    match network {
        "bitcoin_testnet" => Network::Testnet,
        "bitcoin_signet"  => Network::Signet,
        _                 => Network::Bitcoin,
    }
}

pub struct BitcoinAddress(pub Address);

impl BitcoinAddress {
    pub fn from_string(address_str: &str) -> Result<Self> {
        let addr = Address::from_str(address_str)
            .map_err(|e| anyhow::anyhow!("Invalid Bitcoin address: {}", e))?;
        Ok(BitcoinAddress(addr))
    }
}

impl std::fmt::Display for BitcoinAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
