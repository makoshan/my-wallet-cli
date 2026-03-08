
use crate::Result;
use bitcoin::Network;

pub fn network_from_str(s: &str) -> Network {
    match s {
        "bitcoin_testnet" => Network::Testnet,
        "bitcoin_signet"  => Network::Signet,
        _                 => Network::Bitcoin,
    }
}
