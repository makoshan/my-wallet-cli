use crate::Result;
use bitcoin::util::address::Address;
use bitcoin::util::key::PublicKey;
use bitcoin::Network;
use std::str::FromStr;
use tcx_constants::CoinInfo;
use tcx_keystore::Address as AddressTrait;
use tcx_primitive::TypedPublicKey;

pub fn network_from_str(network: &str) -> Network {
    match network {
        "bitcoin_testnet" => Network::Testnet,
        "bitcoin_signet" => Network::Signet,
        _ => Network::Bitcoin,
    }
}

#[derive(Clone, PartialEq, Eq)]
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

impl FromStr for BitcoinAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s)
    }
}

impl AddressTrait for BitcoinAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<Self> {
        let network = match coin.network.as_str() {
            "TESTNET" => Network::Testnet,
            _ => Network::Bitcoin,
        };
        let public_key = PublicKey::from_slice(&public_key.to_bytes())
            .map_err(|e| anyhow::anyhow!("Invalid Bitcoin public key: {}", e))?;
        let address = match coin.seg_wit.as_str() {
            "P2WPKH" => Address::p2shwpkh(&public_key, network)
                .map_err(|e| anyhow::anyhow!("Invalid Bitcoin address: {}", e))?,
            "VERSION_0" => Address::p2wpkh(&public_key, network)
                .map_err(|e| anyhow::anyhow!("Invalid Bitcoin address: {}", e))?,
            _ => Address::p2pkh(&public_key, network),
        };
        Ok(BitcoinAddress(address))
    }

    fn is_valid(address: &str, _coin: &CoinInfo) -> bool {
        BitcoinAddress::from_str(address).is_ok()
    }
}
