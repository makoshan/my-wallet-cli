
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Key, Nonce};
use anyhow::Result;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::path::PathBuf;
use zeroize::Zeroize;
use crate::config::keystore_dir;

/// Read password: from /dev/tty if available (interactive), otherwise from stdin (CI/pipe mode).
pub fn read_password(prompt: &str) -> anyhow::Result<String> {
    use std::io::{self, BufRead};
    // 1. Check WALLET_PASSWORD env var (for AI/CI usage)
    if let Ok(pw) = std::env::var("WALLET_PASSWORD") {
        return Ok(pw);
    }
    // 2. If stdin is not a TTY (pipe), read from stdin
    if !atty::is(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        return Ok(line.trim_end_matches('\n').trim_end_matches('\r').to_string());
    }
    // 3. Interactive TTY: use rpassword
    Ok(rpassword::prompt_password(prompt)?)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletMeta {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub chains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedKeystore {
    meta: WalletMeta,
    salt: String,
    nonce: String,
    ciphertext: String,
}

pub fn save_wallet(name: &str, mnemonic: &str, password: &str) -> Result<String> {
    let id = uuid::Uuid::new_v4().to_string();
    let mut salt = [0u8; 32];
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);
    let mut key_bytes = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 100_000, &mut key_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let ct = cipher.encrypt(Nonce::from_slice(&nonce_bytes), mnemonic.as_bytes())
        .map_err(|e| anyhow::anyhow!("encrypt: {}", e))?;
    key_bytes.zeroize();
    let ks = EncryptedKeystore {
        meta: WalletMeta {
            id: id.clone(), name: name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            chains: vec!["ethereum".into(),"bitcoin".into(),"solana".into()],
        },
        salt: hex::encode(salt), nonce: hex::encode(nonce_bytes), ciphertext: hex::encode(ct),
    };
    let path = keystore_dir().join(format!("{}.json", id));
    std::fs::write(&path, serde_json::to_string_pretty(&ks)?)?;
    Ok(id)
}

pub fn load_mnemonic(id_or_name: &str, password: &str) -> Result<String> {
    let path = find_keystore(id_or_name)?;
    let ks: EncryptedKeystore = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
    let salt = hex::decode(&ks.salt)?;
    let nonce_bytes = hex::decode(&ks.nonce)?;
    let ct = hex::decode(&ks.ciphertext)?;
    let mut key_bytes = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 100_000, &mut key_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let pt = cipher.decrypt(Nonce::from_slice(&nonce_bytes), ct.as_ref())
        .map_err(|_| anyhow::anyhow!("Wrong password or corrupted keystore"))?;
    key_bytes.zeroize();
    Ok(String::from_utf8(pt)?)
}

pub fn list_wallets() -> Result<Vec<WalletMeta>> {
    let dir = keystore_dir();
    if !dir.exists() { return Ok(vec![]); }
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir)? {
        let e = e?;
        if e.path().extension().map(|x| x == "json").unwrap_or(false) {
            if let Ok(ks) = serde_json::from_str::<EncryptedKeystore>(&std::fs::read_to_string(e.path())?) {
                out.push(ks.meta);
            }
        }
    }
    Ok(out)
}

pub fn delete_wallet(id_or_name: &str) -> Result<()> {
    std::fs::remove_file(find_keystore(id_or_name)?)?;
    Ok(())
}

fn find_keystore(id_or_name: &str) -> Result<PathBuf> {
    for e in std::fs::read_dir(keystore_dir())? {
        let e = e?;
        let p = e.path();
        if p.extension().map(|x| x == "json").unwrap_or(false) {
            if let Ok(ks) = serde_json::from_str::<EncryptedKeystore>(&std::fs::read_to_string(&p)?) {
                if ks.meta.id == id_or_name || ks.meta.name == id_or_name { return Ok(p); }
            }
        }
    }
    Err(anyhow::anyhow!("Wallet '{}' not found", id_or_name))
}
