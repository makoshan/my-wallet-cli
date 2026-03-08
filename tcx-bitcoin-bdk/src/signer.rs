use crate::transaction::{BitcoinMessageInput, BitcoinMessageOutput, BitcoinTxInput, BitcoinTxOutput};
use crate::Result;
use bitcoin::hashes::{sha256, Hash};
use hex::ToHex;
use tcx_keystore::{Keystore, MessageSigner, SignatureParameters, TransactionSigner};

impl TransactionSigner<BitcoinTxInput, BitcoinTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        params: &SignatureParameters,
        input: &BitcoinTxInput,
    ) -> tcx_keystore::Result<BitcoinTxOutput> {
        // Decode PSBT from base64
        let psbt_bytes = base64::decode(&input.psbt)
            .map_err(|e| anyhow::anyhow!("Failed to decode PSBT: {}", e))?;

        // Sign the PSBT
        let signature = self.secp256k1_ecdsa_sign_recoverable(&psbt_bytes, &params.derivation_path)?;

        Ok(BitcoinTxOutput {
            signed_psbt: base64::encode(&signature),
            tx_hash: sha256::Hash::hash(&psbt_bytes).to_string(),
        })
    }
}

impl MessageSigner<BitcoinMessageInput, BitcoinMessageOutput> for Keystore {
    fn sign_message(
        &mut self,
        params: &SignatureParameters,
        input: &BitcoinMessageInput,
    ) -> tcx_keystore::Result<BitcoinMessageOutput> {
        // Bitcoin message signing format
        const PREFIX: &str = "\x18Bitcoin Signed Message:\n";
        let message = input.message.as_bytes();
        let len = message.len();
        let len_string = len.to_string();

        let mut bitcoin_message = Vec::with_capacity(PREFIX.len() + len_string.len() + len);
        bitcoin_message.extend_from_slice(PREFIX.as_bytes());
        bitcoin_message.extend_from_slice(len_string.as_bytes());
        bitcoin_message.extend_from_slice(message);

        let message_hash = sha256::Hash::hash(&bitcoin_message);
        let signature = self.secp256k1_ecdsa_sign_recoverable(message_hash.as_ref(), &params.derivation_path)?;

        Ok(BitcoinMessageOutput {
            signature: signature.to_hex(),
            message_hash: message_hash.to_string(),
        })
    }
}
