use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaTxInput {
    /// Base64-encoded serialized Solana transaction
    pub transaction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaTxOutput {
    pub signature: String,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaMessageInput {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaMessageOutput {
    pub signature: String,
    pub message_hash: String,
}
