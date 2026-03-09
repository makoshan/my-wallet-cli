use crate::transaction::{
    BitcoinMessageInput, BitcoinMessageOutput, BitcoinTxInput, BitcoinTxOutput,
};
use bitcoin_hashes::{sha256d, Hash};
use tcx_keystore::{Keystore, MessageSigner, SignatureParameters, Signer, TransactionSigner};

impl TransactionSigner<BitcoinTxInput, BitcoinTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        params: &SignatureParameters,
        input: &BitcoinTxInput,
    ) -> tcx_keystore::Result<BitcoinTxOutput> {
        let psbt_bytes = base64::decode(&input.psbt)
            .map_err(|e| anyhow::anyhow!("Failed to decode PSBT: {}", e))?;

        let hash = sha256d::Hash::hash(&psbt_bytes);
        let sig = self.secp256k1_ecdsa_sign_recoverable(hash.as_ref(), &params.derivation_path)?;

        Ok(BitcoinTxOutput {
            signed_psbt: base64::encode(&sig),
            tx_hash: hash.to_string(),
        })
    }
}

impl MessageSigner<BitcoinMessageInput, BitcoinMessageOutput> for Keystore {
    fn sign_message(
        &mut self,
        params: &SignatureParameters,
        input: &BitcoinMessageInput,
    ) -> tcx_keystore::Result<BitcoinMessageOutput> {
        const PREFIX: &[u8] = b"\x18Bitcoin Signed Message:\n";
        let msg = input.message.as_bytes();
        let mut data = Vec::with_capacity(PREFIX.len() + 10 + msg.len());
        data.extend_from_slice(PREFIX);
        // varint for message length
        let len = msg.len();
        if len < 0xfd {
            data.push(len as u8);
        } else {
            data.push(0xfd);
            data.push((len & 0xff) as u8);
            data.push((len >> 8) as u8);
        }
        data.extend_from_slice(msg);

        let hash = sha256d::Hash::hash(&data);
        let sig = self.secp256k1_ecdsa_sign_recoverable(hash.as_ref(), &params.derivation_path)?;

        Ok(BitcoinMessageOutput {
            signature: hex::encode(&sig),
            message_hash: hash.to_string(),
        })
    }
}
