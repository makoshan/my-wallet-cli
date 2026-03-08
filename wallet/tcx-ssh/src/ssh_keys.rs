
use crate::Result;
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use zeroize::Zeroize;

pub const SSH_DERIVATION_PATH: &str = "m/44'/19999'/0'/0'/0'";

pub struct SshKeypair {
    signing_key: SigningKey,
}

impl Drop for SshKeypair {
    fn drop(&mut self) {
        let mut b = self.signing_key.to_bytes();
        b.zeroize();
    }
}

impl SshKeypair {
    /// Derive from the first 32 bytes of a BIP32 seed.
    pub fn from_seed_bytes(seed: &[u8]) -> Result<Self> {
        if seed.len() < 32 {
            return Err(anyhow::anyhow!("Seed must be at least 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&seed[..32]);
        let sk = SigningKey::from_bytes(&arr);
        Ok(SshKeypair { signing_key: sk })
    }

    fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    /// OpenSSH wire-format public key blob.
    pub fn public_key_blob(&self) -> Vec<u8> {
        let key_type = b"ssh-ed25519";
        let key_bytes = self.verifying_key().to_bytes();
        let mut blob = Vec::new();
        write_string(&mut blob, key_type);
        write_bytes(&mut blob, &key_bytes);
        blob
    }

    /// Single line for `authorized_keys`.
    pub fn authorized_keys_line(&self, comment: &str) -> String {
        let b64 = B64.encode(self.public_key_blob());
        if comment.is_empty() { format!("ssh-ed25519 {}", b64) }
        else { format!("ssh-ed25519 {} {}", b64, comment) }
    }

    /// OpenSSH private key PEM (unencrypted).
    pub fn private_key_openssh(&self, comment: &str) -> String {
        let magic = b"openssh-key-v1\0";
        let pub_blob = self.public_key_blob();
        let secret_bytes = self.signing_key.to_bytes();
        let public_bytes = self.verifying_key().to_bytes();

        let check: u32 = 0x12345678;
        let mut priv_sec = Vec::new();
        priv_sec.extend_from_slice(&check.to_be_bytes());
        priv_sec.extend_from_slice(&check.to_be_bytes());
        write_string(&mut priv_sec, b"ssh-ed25519");
        write_bytes(&mut priv_sec, &public_bytes);
        let mut priv_pub = [0u8; 64];
        priv_pub[..32].copy_from_slice(&secret_bytes);
        priv_pub[32..].copy_from_slice(&public_bytes);
        write_bytes(&mut priv_sec, &priv_pub);
        write_string(&mut priv_sec, comment.as_bytes());
        let mut pad = 1u8;
        while priv_sec.len() % 8 != 0 { priv_sec.push(pad); pad += 1; }

        let mut body = Vec::new();
        body.extend_from_slice(magic);
        write_string(&mut body, b"none");
        write_string(&mut body, b"none");
        write_bytes(&mut body, &[]);
        body.extend_from_slice(&1u32.to_be_bytes());
        write_bytes(&mut body, &pub_blob);
        write_bytes(&mut body, &priv_sec);

        let b64 = B64.encode(&body);
        let mut pem = String::from("-----BEGIN OPENSSH PRIVATE KEY-----\n");
        for chunk in b64.as_bytes().chunks(70) {
            pem.push_str(std::str::from_utf8(chunk).unwrap());
            pem.push('\n');
        }
        pem.push_str("-----END OPENSSH PRIVATE KEY-----\n");
        pem
    }

    /// SHA-256 fingerprint (matches `ssh-keygen -l -E sha256`).
    pub fn fingerprint_sha256(&self) -> String {
        let hash = Sha256::digest(self.public_key_blob());
        let b64 = B64.encode(hash);
        format!("SHA256:{}", b64.trim_end_matches('='))
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.signing_key.sign(data).to_bytes().to_vec()
    }
}

fn write_string(buf: &mut Vec<u8>, s: &[u8]) {
    buf.extend_from_slice(&(s.len() as u32).to_be_bytes());
    buf.extend_from_slice(s);
}
fn write_bytes(buf: &mut Vec<u8>, b: &[u8]) {
    buf.extend_from_slice(&(b.len() as u32).to_be_bytes());
    buf.extend_from_slice(b);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn seed() -> Vec<u8> { (0u8..64).collect() }

    #[test]
    fn test_authorized_keys_line() {
        let kp = SshKeypair::from_seed_bytes(&seed()).unwrap();
        let line = kp.authorized_keys_line("test");
        assert!(line.starts_with("ssh-ed25519 "));
        assert!(line.ends_with("test"));
    }

    #[test]
    fn test_fingerprint() {
        let kp = SshKeypair::from_seed_bytes(&seed()).unwrap();
        assert!(kp.fingerprint_sha256().starts_with("SHA256:"));
    }

    #[test]
    fn test_pem() {
        let kp = SshKeypair::from_seed_bytes(&seed()).unwrap();
        let pem = kp.private_key_openssh("ci");
        assert!(pem.contains("BEGIN OPENSSH PRIVATE KEY"));
        assert!(pem.contains("END OPENSSH PRIVATE KEY"));
    }
}
