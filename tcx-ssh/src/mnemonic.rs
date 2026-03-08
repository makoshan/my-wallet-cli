use crate::Result;
use bip39::{Language, Mnemonic as Bip39Mnemonic, MnemonicType, Seed};
use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MnemonicLength {
    Words12,
    Words18,
    Words24,
}

impl MnemonicLength {
    pub fn word_count(&self) -> usize {
        match self {
            MnemonicLength::Words12 => 12,
            MnemonicLength::Words18 => 18,
            MnemonicLength::Words24 => 24,
        }
    }

    pub fn from_word_count(count: usize) -> Result<Self> {
        match count {
            12 => Ok(MnemonicLength::Words12),
            18 => Ok(MnemonicLength::Words18),
            24 => Ok(MnemonicLength::Words24),
            _ => Err(anyhow::anyhow!("Invalid mnemonic length: {}", count)),
        }
    }
}

#[derive(Clone, ZeroizeOnDrop)]
pub struct MnemonicHandler {
    #[zeroize(skip)]
    phrase: String,
    #[zeroize(skip)]
    entropy: Vec<u8>,
}

impl MnemonicHandler {
    /// 创建一个新的随机助记词
    pub fn new(length: MnemonicLength) -> Result<Self> {
        let mnemonic_type = match length {
            MnemonicLength::Words12 => MnemonicType::Words12,
            MnemonicLength::Words18 => MnemonicType::Words18,
            MnemonicLength::Words24 => MnemonicType::Words24,
        };

        let mnemonic = Bip39Mnemonic::new(mnemonic_type, Language::English);
        let phrase = mnemonic.phrase().to_string();
        let entropy = mnemonic.entropy().to_vec();

        Ok(Self { phrase, entropy })
    }

    /// 从现有的助记词短语创建
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        let mnemonic = Bip39Mnemonic::from_phrase(phrase, Language::English)
            .map_err(|e| anyhow::anyhow!("Invalid mnemonic phrase: {}", e))?;

        let entropy = mnemonic.entropy().to_vec();

        Ok(Self {
            phrase: phrase.to_string(),
            entropy,
        })
    }

    /// 获取助记词短语
    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    /// 获取熵
    pub fn entropy(&self) -> &[u8] {
        &self.entropy
    }

    /// 从助记词生成种子
    pub fn to_seed(&self, passphrase: Option<&str>) -> Result<Vec<u8>> {
        let mnemonic = Bip39Mnemonic::from_phrase(&self.phrase, Language::English)
            .map_err(|e| anyhow::anyhow!("Invalid mnemonic phrase: {}", e))?;

        let passphrase = passphrase.unwrap_or("");
        let seed = Seed::new(&mnemonic, passphrase);

        Ok(seed.as_bytes().to_vec())
    }

    /// 验证助记词是否有效
    pub fn is_valid(phrase: &str) -> bool {
        Bip39Mnemonic::from_phrase(phrase, Language::English).is_ok()
    }

    /// 获取助记词的字数
    pub fn word_count(&self) -> usize {
        self.phrase.split_whitespace().count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MnemonicInfo {
    pub phrase: String,
    pub word_count: usize,
    pub entropy_hex: String,
}

impl From<&MnemonicHandler> for MnemonicInfo {
    fn from(handler: &MnemonicHandler) -> Self {
        MnemonicInfo {
            phrase: handler.phrase.clone(),
            word_count: handler.word_count(),
            entropy_hex: hex::encode(&handler.entropy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mnemonic() {
        let mnemonic = MnemonicHandler::new(MnemonicLength::Words12).unwrap();
        assert_eq!(mnemonic.word_count(), 12);
    }

    #[test]
    fn test_mnemonic_from_phrase() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = MnemonicHandler::from_phrase(phrase).unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
    }

    #[test]
    fn test_mnemonic_to_seed() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = MnemonicHandler::from_phrase(phrase).unwrap();
        let seed = mnemonic.to_seed(None).unwrap();
        assert_eq!(seed.len(), 64);
    }
}
