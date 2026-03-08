
use crate::address::EvmAddress;
use crate::transaction::{EvmMessageInput, EvmMessageOutput, EvmTxInput, EvmTxOutput};
use crate::Result;
use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use zeroize::Zeroize;

pub struct EvmKeystore {
    secret_key: SecretKey,
}

impl Drop for EvmKeystore {
    fn drop(&mut self) {
        let mut bytes = self.secret_key.secret_bytes();
        bytes.zeroize();
    }
}

impl EvmKeystore {
    pub fn from_secret_bytes(bytes: &[u8]) -> Result<Self> {
        let sk = SecretKey::from_slice(bytes)
            .map_err(|e| anyhow::anyhow!("Invalid secret key: {}", e))?;
        Ok(EvmKeystore { secret_key: sk })
    }

    pub fn address(&self) -> Result<EvmAddress> {
        let secp = Secp256k1::new();
        let pubkey = secp256k1::PublicKey::from_secret_key(&secp, &self.secret_key);
        let pubkey_bytes = pubkey.serialize_uncompressed();
        EvmAddress::from_public_key_bytes(&pubkey_bytes)
    }

    pub fn sign_message(&self, input: &EvmMessageInput) -> Result<EvmMessageOutput> {
        let prefix = format!("\x19Ethereum Signed Message:\n{}", input.message.len());
        let mut data = prefix.into_bytes();
        data.extend_from_slice(input.message.as_bytes());
        let hash = Keccak256::digest(&data);

        let secp = Secp256k1::new();
        let msg = Message::from_digest_slice(hash.as_ref())
            .map_err(|e| anyhow::anyhow!("msg: {}", e))?;
        let sig = secp.sign_ecdsa_recoverable(&msg, &self.secret_key);
        let (rec_id, sig_bytes) = sig.serialize_compact();
        let mut full_sig = [0u8; 65];
        full_sig[..64].copy_from_slice(&sig_bytes);
        full_sig[64] = rec_id.to_i32() as u8 + 27;

        Ok(EvmMessageOutput {
            signature: format!("0x{}", hex::encode(full_sig)),
            message_hash: format!("0x{}", hex::encode(hash)),
        })
    }

    pub fn sign_transaction(&self, input: &EvmTxInput) -> Result<EvmTxOutput> {
        let tx_json = serde_json::to_string(input)?;
        let hash = Keccak256::digest(tx_json.as_bytes());

        let secp = Secp256k1::new();
        let msg = Message::from_digest_slice(hash.as_ref())
            .map_err(|e| anyhow::anyhow!("msg: {}", e))?;
        let sig = secp.sign_ecdsa_recoverable(&msg, &self.secret_key);
        let (rec_id, sig_bytes) = sig.serialize_compact();
        let mut full_sig = [0u8; 65];
        full_sig[..64].copy_from_slice(&sig_bytes);
        full_sig[64] = rec_id.to_i32() as u8 + 27;

        Ok(EvmTxOutput {
            signature: format!("0x{}", hex::encode(full_sig)),
            tx_hash: format!("0x{}", hex::encode(hash)),
        })
    }
}
