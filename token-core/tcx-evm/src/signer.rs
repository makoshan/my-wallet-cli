use crate::transaction::{EvmMessageInput, EvmMessageOutput, EvmTxInput, EvmTxOutput};
use crate::Result;
use alloy_primitives::Signature;
use hex::ToHex;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use tcx_keystore::{Keystore, MessageSigner, SignatureParameters, TransactionSigner};

fn keccak256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn hash_message<T: AsRef<[u8]>>(message: T) -> Vec<u8> {
    const PREFIX: &str = "\x19Ethereum Signed Message:\n";

    let message = message.as_ref();
    let len = message.len();
    let len_string = len.to_string();

    let mut eth_message = Vec::with_capacity(PREFIX.len() + len_string.len() + len);
    eth_message.extend_from_slice(PREFIX.as_bytes());
    eth_message.extend_from_slice(len_string.as_bytes());
    eth_message.extend_from_slice(message);

    keccak256(&eth_message)
}

impl TransactionSigner<EvmTxInput, EvmTxOutput> for Keystore {
    fn sign_transaction(
        &mut self,
        params: &SignatureParameters,
        input: &EvmTxInput,
    ) -> tcx_keystore::Result<EvmTxOutput> {
        // Get the private key from keystore
        let private_key_bytes = self.secp256k1_ecdsa_sign_recoverable(
            &[0u8; 32], // placeholder hash
            &params.derivation_path,
        )?;

        // Sign the transaction
        let tx_hash = keccak256(input.to.as_bytes());
        let signature = self.secp256k1_ecdsa_sign_recoverable(&tx_hash, &params.derivation_path)?;

        Ok(EvmTxOutput {
            signature: signature.to_hex(),
            tx_hash: tx_hash.to_hex(),
        })
    }
}

impl MessageSigner<EvmMessageInput, EvmMessageOutput> for Keystore {
    fn sign_message(
        &mut self,
        params: &SignatureParameters,
        input: &EvmMessageInput,
    ) -> tcx_keystore::Result<EvmMessageOutput> {
        let message_hash = hash_message(&input.message);
        let signature = self.secp256k1_ecdsa_sign_recoverable(&message_hash, &params.derivation_path)?;

        Ok(EvmMessageOutput {
            signature: signature.to_hex(),
            message_hash: message_hash.to_hex(),
        })
    }
}
