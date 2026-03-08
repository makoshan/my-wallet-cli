use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinTxInput {
    /// Base64-encoded PSBT
    pub psbt: String,
    pub network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinTxOutput {
    pub signed_psbt: String,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinMessageInput {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinMessageOutput {
    pub signature: String,
    pub message_hash: String,
}
