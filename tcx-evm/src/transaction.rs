use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmTxInput {
    pub to: String,
    pub value: String,
    pub data: Option<String>,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<String>,
    pub nonce: Option<u64>,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmTxOutput {
    pub signature: String,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmMessageInput {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmMessageOutput {
    pub signature: String,
    pub message_hash: String,
}
