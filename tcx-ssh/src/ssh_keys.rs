use crate::mnemonic::MnemonicHandler;
use crate::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct SshKeyGenerator {
    signing_key: SigningKey,
    #[zeroize(skip)]
    verifying_key: VerifyingKey,
    #[zeroize(skip)]
    private_key_openssh: String,
    #[zeroize(skip)]
    public_key_openssh: String,
}

impl SshKeyGenerator {
    /// 从助记词生成 SSH 密钥对
    pub fn from_mnemonic(
        mnemonic: &MnemonicHandler,
        passphrase: Option<&str>,
        comment: Option<&str>,
    ) -> Result<Self> {
        let seed = mnemonic.to_seed(passphrase)?;

        // 使用种子的前 32 字节作为 Ed25519 密钥
        if seed.len() < 32 {
            return Err(anyhow::anyhow!("Seed is too short"));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&seed[..32]);

        let signing_key = SigningKey::from(key_bytes);
        let verifying_key = signing_key.verifying_key();

        // 格式化为 OpenSSH 格式
        let private_key_openssh = format_openssh_private_key(&signing_key, comment)?;
        let public_key_openssh = format_openssh_public_key(&verifying_key, comment)?;

        Ok(Self {
            signing_key,
            verifying_key,
            private_key_openssh,
            public_key_openssh,
        })
    }

    /// 从原始种子生成 SSH 密钥对
    pub fn from_seed(seed: &[u8], comment: Option<&str>) -> Result<Self> {
        if seed.len() < 32 {
            return Err(anyhow::anyhow!("Seed is too short"));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&seed[..32]);

        let signing_key = SigningKey::from(key_bytes);
        let verifying_key = signing_key.verifying_key();

        let private_key_openssh = format_openssh_private_key(&signing_key, comment)?;
        let public_key_openssh = format_openssh_public_key(&verifying_key, comment)?;

        Ok(Self {
            signing_key,
            verifying_key,
            private_key_openssh,
            public_key_openssh,
        })
    }

    /// 获取 OpenSSH 格式的私钥
    pub fn private_key_openssh(&self) -> &str {
        &self.private_key_openssh
    }

    /// 获取 OpenSSH 格式的公钥
    pub fn public_key_openssh(&self) -> &str {
        &self.public_key_openssh
    }

    /// 获取公钥的 Base64 编码
    pub fn public_key_base64(&self) -> String {
        BASE64.encode(self.verifying_key.to_bytes())
    }

    /// 获取公钥的十六进制编码
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    /// 对消息进行签名
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }

    /// 验证签名
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }

        let sig_bytes: [u8; 64] = match signature.try_into() {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes);
        self.verifying_key.verify(message, &signature).is_ok()
    }

    /// 保存密钥对到文件
    pub fn save_to_files(&self, private_key_path: impl AsRef<Path>) -> Result<(String, String)> {
        let private_key_path = private_key_path.as_ref();
        let public_key_path = private_key_path.with_extension("pub");

        // 创建目录
        if let Some(parent) = private_key_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 写入私钥和公钥
        fs::write(&private_key_path, &self.private_key_openssh)?;
        fs::write(&public_key_path, &self.public_key_openssh)?;

        // 设置私钥文件权限为 0600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&private_key_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&private_key_path, perms)?;
        }

        Ok((
            private_key_path.to_string_lossy().to_string(),
            public_key_path.to_string_lossy().to_string(),
        ))
    }

    /// 获取公钥的 MD5 指纹
    pub fn md5_fingerprint(&self) -> String {
        use md5;
        let public_key_bytes = self.verifying_key.to_bytes();
        let digest = md5::compute(&public_key_bytes);
        let hex = format!("{:x}", digest);

        // 格式化为 OpenSSH 指纹格式
        let mut fingerprint = String::new();
        for (i, chunk) in hex.chars().collect::<Vec<_>>().chunks(2).enumerate() {
            if i > 0 {
                fingerprint.push(':');
            }
            fingerprint.push(chunk[0]);
            fingerprint.push(chunk[1]);
        }
        format!("MD5:{}", fingerprint)
    }

    /// 获取公钥的 SHA256 指纹
    pub fn sha256_fingerprint(&self) -> String {
        let public_key_bytes = self.verifying_key.to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        let digest = hasher.finalize();
        format!("SHA256:{}", BASE64.encode(digest).trim_end_matches('='))
    }
}

/// 格式化为 OpenSSH 私钥格式
fn format_openssh_private_key(signing_key: &SigningKey, comment: Option<&str>) -> Result<String> {
    let public_key_bytes = signing_key.verifying_key().to_bytes();
    let private_key_bytes = signing_key.to_bytes();

    let comment = comment.unwrap_or("Generated SSH key");

    // OpenSSH 私钥格式
    let mut result = String::from("-----BEGIN OPENSSH PRIVATE KEY-----\n");

    // 构建私钥数据
    let mut key_data = Vec::new();

    // Magic number
    key_data.extend_from_slice(b"openssh-key-v1\0");

    // Cipher name (none)
    append_string(&mut key_data, b"none");

    // KDF name (none)
    append_string(&mut key_data, b"none");

    // KDF options (empty)
    append_string(&mut key_data, b"");

    // Number of keys
    key_data.extend_from_slice(&1u32.to_be_bytes());

    // Public key
    let mut pub_key = Vec::new();
    append_string(&mut pub_key, b"ssh-ed25519");
    append_string(&mut pub_key, &public_key_bytes);
    append_string(&mut key_data, &pub_key);

    // Private key section
    let mut priv_section = Vec::new();

    // Check int (random)
    let check_int = 0u32;
    priv_section.extend_from_slice(&check_int.to_be_bytes());
    priv_section.extend_from_slice(&check_int.to_be_bytes());

    // Key type
    append_string(&mut priv_section, b"ssh-ed25519");

    // Public key
    append_string(&mut priv_section, &public_key_bytes);

    // Private key + public key
    let mut full_key = Vec::new();
    full_key.extend_from_slice(&private_key_bytes);
    full_key.extend_from_slice(&public_key_bytes);
    append_string(&mut priv_section, &full_key);

    // Comment
    append_string(&mut priv_section, comment.as_bytes());

    // Padding
    let padding_len = 8 - (priv_section.len() % 8);
    for i in 1..=padding_len {
        priv_section.push(i as u8);
    }

    append_string(&mut key_data, &priv_section);

    // Encode to base64
    let encoded = BASE64.encode(&key_data);

    // Add base64 in 70-character lines
    for chunk in encoded.chars().collect::<Vec<_>>().chunks(70) {
        result.push_str(&chunk.iter().collect::<String>());
        result.push('\n');
    }

    result.push_str("-----END OPENSSH PRIVATE KEY-----\n");

    Ok(result)
}

/// 格式化为 OpenSSH 公钥格式
fn format_openssh_public_key(verifying_key: &VerifyingKey, comment: Option<&str>) -> Result<String> {
    let public_key_bytes = verifying_key.to_bytes();
    let comment = comment.unwrap_or("Generated SSH key");

    let mut key_data = Vec::new();
    append_string(&mut key_data, b"ssh-ed25519");
    append_string(&mut key_data, &public_key_bytes);

    let encoded = BASE64.encode(&key_data);
    Ok(format!("ssh-ed25519 {} {}", encoded, comment))
}

/// 辅助函数：追加字符串到向量
fn append_string(vec: &mut Vec<u8>, data: &[u8]) {
    vec.extend_from_slice(&(data.len() as u32).to_be_bytes());
    vec.extend_from_slice(data);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyInfo {
    pub public_key: String,
    pub public_key_base64: String,
    pub public_key_hex: String,
    pub fingerprint_md5: String,
    pub fingerprint_sha256: String,
    pub comment: String,
}

impl From<&SshKeyGenerator> for SshKeyInfo {
    fn from(generator: &SshKeyGenerator) -> Self {
        SshKeyInfo {
            public_key: generator.public_key_openssh.clone(),
            public_key_base64: generator.public_key_base64(),
            public_key_hex: generator.public_key_hex(),
            fingerprint_md5: generator.md5_fingerprint(),
            fingerprint_sha256: generator.sha256_fingerprint(),
            comment: "Generated SSH key".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_key_from_seed() {
        let seed = vec![0u8; 64];
        let generator = SshKeyGenerator::from_seed(&seed, None).unwrap();
        assert!(!generator.public_key_openssh().is_empty());
    }

    #[test]
    fn test_ssh_key_sign_verify() {
        let seed = vec![1u8; 64];
        let generator = SshKeyGenerator::from_seed(&seed, None).unwrap();

        let message = b"test message";
        let signature = generator.sign(message);
        assert!(generator.verify(message, &signature));
    }
}
