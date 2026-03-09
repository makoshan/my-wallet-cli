use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmTxInput {
    pub to: String,
    pub value: String, // hex-encoded wei
    pub data: String,  // hex-encoded calldata
    pub nonce: u64,
    pub gas_limit: u64,
    pub max_fee_per_gas: String,
    pub max_priority_fee_per_gas: String,
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
