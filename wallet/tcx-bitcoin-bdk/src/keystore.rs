
use crate::address::network_from_str;
use crate::transaction::{BitcoinMessageInput, BitcoinMessageOutput, BitcoinTxInput, BitcoinTxOutput};
use crate::Result;
use bitcoin::secp256k1::{All, Secp256k1, SecretKey};
use bitcoin::hashes::{sha256d, Hash};
use bitcoin::{Address, Network, PublicKey};
use zeroize::Zeroize;

pub struct BitcoinKeystore {
    secret_key: SecretKey,
    network: Network,
}

impl Drop for BitcoinKeystore {
    fn drop(&mut self) {
        let mut b = self.secret_key.secret_bytes();
        b.zeroize();
    }
}

impl BitcoinKeystore {
    pub fn from_secret_bytes(bytes: &[u8], network: &str) -> Result<Self> {
        let sk = SecretKey::from_slice(bytes)
            .map_err(|e| anyhow::anyhow!("Invalid secret key: {}", e))?;
        Ok(BitcoinKeystore { secret_key: sk, network: network_from_str(network) })
    }

    pub fn address(&self) -> Result<String> {
        let secp = Secp256k1::new();
        let pubkey_raw = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &self.secret_key);
        let pubkey = PublicKey::new(pubkey_raw);
        let addr = Address::p2wpkh(&pubkey, self.network)
            .map_err(|e| anyhow::anyhow!("p2wpkh: {}", e))?;
        Ok(addr.to_string())
    }

    pub fn sign_message(&self, input: &BitcoinMessageInput) -> Result<BitcoinMessageOutput> {
        const PREFIX: &[u8] = b"\x18Bitcoin Signed Message:\n";
        let msg = input.message.as_bytes();
        let mut data = Vec::new();
        data.extend_from_slice(PREFIX);
        data.push(msg.len() as u8);
        data.extend_from_slice(msg);
        let hash = sha256d::Hash::hash(&data);

        let secp = Secp256k1::new();
        let msg_obj = bitcoin::secp256k1::Message::from_slice(hash.as_ref())
            .map_err(|e| anyhow::anyhow!("msg: {}", e))?;
        let sig = secp.sign_ecdsa_recoverable(&msg_obj, &self.secret_key);
        let (rec_id, sig_bytes) = sig.serialize_compact();
        let mut full = [0u8; 65];
        full[0] = 31 + rec_id.to_i32() as u8;
        full[1..].copy_from_slice(&sig_bytes);

        Ok(BitcoinMessageOutput {
            signature: base64::encode(full),
            message_hash: hash.to_string(),
        })
    }

    pub fn sign_transaction(&self, input: &BitcoinTxInput) -> Result<BitcoinTxOutput> {
        let psbt_bytes = base64::decode(&input.psbt)
            .map_err(|e| anyhow::anyhow!("decode psbt: {}", e))?;
        let hash = sha256d::Hash::hash(&psbt_bytes);

        let secp = Secp256k1::new();
        let msg = bitcoin::secp256k1::Message::from_slice(hash.as_ref())
            .map_err(|e| anyhow::anyhow!("msg: {}", e))?;
        let sig = secp.sign_ecdsa(&msg, &self.secret_key);

        Ok(BitcoinTxOutput {
            signed_psbt: base64::encode(sig.serialize_der()),
            tx_hash: hash.to_string(),
        })
    }
}
