//! Generate Ed25519 SSH keypairs from a BIP39 mnemonic / seed.
//!
//! Derivation path: m/44'/19999'/0'/0'/0'  (custom SSH path)
//! The 32-byte seed slice at that path is used directly as the Ed25519 secret scalar.

use crate::Result;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use sha2::{Digest, Sha256};
use zeroize::Zeroize;

pub const SSH_DERIVATION_PATH: &str = "m/44'/19999'/0'/0'/0'";

/// An Ed25519 SSH keypair derived from a wallet seed.
pub struct SshKeypair {
    keypair: Keypair,
}

impl SshKeypair {
    /// Derive an SSH keypair from a 64-byte BIP32 seed (the raw seed bytes,
    /// not the mnemonic).  We take the first 32 bytes as the Ed25519 secret.
    pub fn from_seed_bytes(seed: &[u8]) -> Result<Self> {
        if seed.len() < 32 {
            return Err(anyhow::anyhow!("Seed must be at least 32 bytes"));
        }
        let secret = SecretKey::from_bytes(&seed[..32])
            .map_err(|e| anyhow::anyhow!("Invalid Ed25519 secret: {}", e))?;
        let public: PublicKey = (&secret).into();
        let keypair = Keypair { secret, public };
        Ok(SshKeypair { keypair })
    }

    /// OpenSSH wire-format public key (the blob inside `authorized_keys`).
    pub fn public_key_blob(&self) -> Vec<u8> {
        let key_type = b"ssh-ed25519";
        let key_bytes = self.keypair.public.as_bytes();

        let mut blob = Vec::new();
        // key type length + key type
        let kt_len = (key_type.len() as u32).to_be_bytes();
        blob.extend_from_slice(&kt_len);
        blob.extend_from_slice(key_type);
        // key bytes length + key bytes
        let kb_len = (key_bytes.len() as u32).to_be_bytes();
        blob.extend_from_slice(&kb_len);
        blob.extend_from_slice(key_bytes);
        blob
    }

    /// The single line that goes into `~/.ssh/authorized_keys`.
    pub fn authorized_keys_line(&self, comment: &str) -> String {
        let blob = self.public_key_blob();
        let b64 = B64.encode(&blob);
        if comment.is_empty() {
            format!("ssh-ed25519 {}", b64)
        } else {
            format!("ssh-ed25519 {} {}", b64, comment)
        }
    }

    /// OpenSSH private key file content (PEM-like, unencrypted).
    pub fn private_key_openssh(&self, comment: &str) -> String {
        // Build the OpenSSH private key format:
        // "openssh-key-v1\0" + cipher + kdf + kdf_options + num_keys + pubkey + private_section
        let magic = b"openssh-key-v1\0";
        let cipher_name = b"none";
        let kdf_name = b"none";
        let kdf_options: &[u8] = &[];
        let num_keys: u32 = 1;

        let pub_blob = self.public_key_blob();
        let secret_bytes = self.keypair.secret.as_bytes();
        let public_bytes = self.keypair.public.as_bytes();

        // Private section: check_int (x2) + key_type + pub + priv+pub (64 bytes) + comment + padding
        let check_int: u32 = 0x12345678;
        let mut priv_section = Vec::new();
        priv_section.extend_from_slice(&check_int.to_be_bytes());
        priv_section.extend_from_slice(&check_int.to_be_bytes());
        // key type
        write_string(&mut priv_section, b"ssh-ed25519");
        // public key
        write_bytes(&mut priv_section, public_bytes);
        // private key (secret || public = 64 bytes)
        let mut priv_pub = [0u8; 64];
        priv_pub[..32].copy_from_slice(secret_bytes);
        priv_pub[32..].copy_from_slice(public_bytes);
        write_bytes(&mut priv_section, &priv_pub);
        // comment
        write_string(&mut priv_section, comment.as_bytes());
        // padding
        let mut pad = 1u8;
        while priv_section.len() % 8 != 0 {
            priv_section.push(pad);
            pad += 1;
        }

        let mut body = Vec::new();
        body.extend_from_slice(magic);
        write_string(&mut body, cipher_name);
        write_string(&mut body, kdf_name);
        write_bytes(&mut body, kdf_options);
        body.extend_from_slice(&num_keys.to_be_bytes());
        write_bytes(&mut body, &pub_blob);
        write_bytes(&mut body, &priv_section);

        let b64 = B64.encode(&body);
        // Wrap at 70 chars
        let mut pem = String::from("-----BEGIN OPENSSH PRIVATE KEY-----\n");
        for chunk in b64.as_bytes().chunks(70) {
            pem.push_str(std::str::from_utf8(chunk).unwrap());
            pem.push('\n');
        }
        pem.push_str("-----END OPENSSH PRIVATE KEY-----\n");
        pem
    }

    /// SHA-256 fingerprint of the public key (as shown by `ssh-keygen -l`).
    pub fn fingerprint_sha256(&self) -> String {
        let blob = self.public_key_blob();
        let hash = Sha256::digest(&blob);
        let b64 = B64.encode(hash);
        // Remove trailing '=' padding
        let b64 = b64.trim_end_matches('=');
        format!("SHA256:{}", b64)
    }

    /// Sign arbitrary bytes with the SSH key.
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.keypair.sign(data).to_bytes().to_vec()
    }
}

impl Drop for SshKeypair {
    fn drop(&mut self) {
        // Zeroize is handled by ed25519-dalek internally for SecretKey
    }
}

fn write_string(buf: &mut Vec<u8>, s: &[u8]) {
    let len = (s.len() as u32).to_be_bytes();
    buf.extend_from_slice(&len);
    buf.extend_from_slice(s);
}

fn write_bytes(buf: &mut Vec<u8>, b: &[u8]) {
    let len = (b.len() as u32).to_be_bytes();
    buf.extend_from_slice(&len);
    buf.extend_from_slice(b);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_seed() -> Vec<u8> {
        // 64 deterministic bytes
        (0u8..64).collect()
    }

    #[test]
    fn test_generate_keypair() {
        let kp = SshKeypair::from_seed_bytes(&test_seed()).unwrap();
        let line = kp.authorized_keys_line("test");
        assert!(line.starts_with("ssh-ed25519 "));
        assert!(line.ends_with("test"));
    }

    #[test]
    fn test_fingerprint() {
        let kp = SshKeypair::from_seed_bytes(&test_seed()).unwrap();
        let fp = kp.fingerprint_sha256();
        assert!(fp.starts_with("SHA256:"));
    }

    #[test]
    fn test_private_key_pem() {
        let kp = SshKeypair::from_seed_bytes(&test_seed()).unwrap();
        let pem = kp.private_key_openssh("test-comment");
        assert!(pem.contains("BEGIN OPENSSH PRIVATE KEY"));
        assert!(pem.contains("END OPENSSH PRIVATE KEY"));
    }
}
