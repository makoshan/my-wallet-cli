
use crate::address::SolanaAddress;
use crate::transaction::{SolanaMessageInput, SolanaMessageOutput, SolanaTxInput, SolanaTxOutput};
use crate::Result;
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};
use zeroize::Zeroize;

pub struct SolanaKeystore {
    signing_key: SigningKey,
}

impl Drop for SolanaKeystore {
    fn drop(&mut self) {
        let mut b = self.signing_key.to_bytes();
        b.zeroize();
    }
}

impl SolanaKeystore {
    pub fn from_secret_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 32 {
            return Err(anyhow::anyhow!("need 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes[..32]);
        let sk = SigningKey::from_bytes(&arr);
        Ok(SolanaKeystore { signing_key: sk })
    }

    pub fn address(&self) -> SolanaAddress {
        let vk = self.signing_key.verifying_key();
        SolanaAddress::from_bytes(vk.to_bytes())
    }

    pub fn sign_message(&self, input: &SolanaMessageInput) -> Result<SolanaMessageOutput> {
        let msg = input.message.as_bytes();
        let sig = self.signing_key.sign(msg);
        let hash = Sha256::digest(msg);
        Ok(SolanaMessageOutput {
            signature: base64::encode(sig.to_bytes()),
            message_hash: hex::encode(hash),
        })
    }

    pub fn sign_transaction(&self, input: &SolanaTxInput) -> Result<SolanaTxOutput> {
        let tx_bytes = base64::decode(&input.transaction)
            .map_err(|e| anyhow::anyhow!("decode tx: {}", e))?;
        let sig = self.signing_key.sign(&tx_bytes);
        let hash = Sha256::digest(&tx_bytes);
        Ok(SolanaTxOutput {
            signature: base64::encode(sig.to_bytes()),
            tx_hash: hex::encode(hash),
        })
    }
}
