use crate::transaction::{SolanaMessageInput, SolanaMessageOutput, SolanaTxInput, SolanaTxOutput};
use crate::Result;
use hex::ToHex;
use sha2::{Digest, Sha256};
use tcx_keystore::{Keystore, MessageSigner, SignatureParameters, TransactionSigner};

impl TransactionSigner<SolanaTxInput, SolanaTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        params: &SignatureParameters,
        input: &SolanaTxInput,
    ) -> tcx_keystore::Result<SolanaTxOutput> {
        // Decode transaction from base64
        let tx_bytes = base64::decode(&input.transaction)
            .map_err(|e| anyhow::anyhow!("Failed to decode transaction: {}", e))?;

        // Sign the transaction
        let signature = self.secp256k1_ecdsa_sign_recoverable(&tx_bytes, &params.derivation_path)?;

        let mut hasher = Sha256::new();
        hasher.update(&tx_bytes);
        let tx_hash = hasher.finalize();

        Ok(SolanaTxOutput {
            signature: signature.to_hex(),
            tx_hash: tx_hash.to_hex(),
        })
    }
}

impl MessageSigner<SolanaMessageInput, SolanaMessageOutput> for Keystore {
    fn sign_message(
        &mut self,
        params: &SignatureParameters,
        input: &SolanaMessageInput,
    ) -> tcx_keystore::Result<SolanaMessageOutput> {
        // Solana message signing format
        const PREFIX: &str = "Solana Signed Message:\n";
        let message = input.message.as_bytes();
        let len = message.len();
        let len_string = len.to_string();

        let mut solana_message = Vec::with_capacity(PREFIX.len() + len_string.len() + len);
        solana_message.extend_from_slice(PREFIX.as_bytes());
        solana_message.extend_from_slice(len_string.as_bytes());
        solana_message.extend_from_slice(message);

        let mut hasher = Sha256::new();
        hasher.update(&solana_message);
        let message_hash = hasher.finalize();

        let signature = self.secp256k1_ecdsa_sign_recoverable(message_hash.as_ref(), &params.derivation_path)?;

        Ok(SolanaMessageOutput {
            signature: signature.to_hex(),
            message_hash: message_hash.to_hex(),
        })
    }
}
