use crate::transaction::{EvmMessageInput, EvmMessageOutput, EvmTxInput, EvmTxOutput};
use sha3::{Digest, Keccak256};
use tcx_keystore::{Keystore, MessageSigner, SignatureParameters, Signer, TransactionSigner};

impl TransactionSigner<EvmTxInput, EvmTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        params: &SignatureParameters,
        input: &EvmTxInput,
    ) -> tcx_keystore::Result<EvmTxOutput> {
        // Build RLP-encoded EIP-1559 transaction hash
        // For simplicity we hash the JSON representation; a production impl
        // would do proper RLP encoding.
        let tx_json =
            serde_json::to_string(input).map_err(|e| anyhow::anyhow!("serialize tx: {}", e))?;
        let hash = Keccak256::digest(tx_json.as_bytes());

        let sig = self.secp256k1_ecdsa_sign_recoverable(hash.as_ref(), &params.derivation_path)?;
        Ok(EvmTxOutput {
            signature: hex::encode(&sig),
            tx_hash: hex::encode(hash),
        })
    }
}

impl MessageSigner<EvmMessageInput, EvmMessageOutput> for Keystore {
    fn sign_message(
        &mut self,
        params: &SignatureParameters,
        input: &EvmMessageInput,
    ) -> tcx_keystore::Result<EvmMessageOutput> {
        // EIP-191 personal_sign prefix
        let prefix = format!("\x19Ethereum Signed Message:\n{}", input.message.len());
        let mut data = prefix.into_bytes();
        data.extend_from_slice(input.message.as_bytes());
        let hash = Keccak256::digest(&data);

        let sig = self.secp256k1_ecdsa_sign_recoverable(hash.as_ref(), &params.derivation_path)?;
        Ok(EvmMessageOutput {
            signature: hex::encode(&sig),
            message_hash: hex::encode(hash),
        })
    }
}
