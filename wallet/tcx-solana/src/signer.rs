use crate::transaction::{
    SolanaMessageInput, SolanaMessageOutput, SolanaTxInput, SolanaTxOutput,
};
use sha2::{Digest, Sha256};
use tcx_keystore::{Keystore, MessageSigner, SignatureParameters, Signer, TransactionSigner};

impl TransactionSigner<SolanaTxInput, SolanaTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        params: &SignatureParameters,
        input: &SolanaTxInput,
    ) -> tcx_keystore::Result<SolanaTxOutput> {
        let tx_bytes = base64::decode(&input.transaction)
            .map_err(|e| anyhow::anyhow!("Failed to decode transaction: {}", e))?;

        // Solana uses Ed25519 for signing
        let sig = self.ed25519_sign(&tx_bytes, &params.derivation_path)?;

        let hash = Sha256::digest(&tx_bytes);
        Ok(SolanaTxOutput {
            signature: base64::encode(&sig),
            tx_hash: hex::encode(hash),
        })
    }
}

impl MessageSigner<SolanaMessageInput, SolanaMessageOutput> for Keystore {
    fn sign_message(
        &mut self,
        params: &SignatureParameters,
        input: &SolanaMessageInput,
    ) -> tcx_keystore::Result<SolanaMessageOutput> {
        let msg_bytes = input.message.as_bytes();
        let sig = self.ed25519_sign(msg_bytes, &params.derivation_path)?;

        let hash = Sha256::digest(msg_bytes);
        Ok(SolanaMessageOutput {
            signature: base64::encode(&sig),
            message_hash: hex::encode(hash),
        })
    }
}
