use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default: DefaultConfig,
    pub chains: HashMap<String, ChainConfig>,
    pub keystore: KeystoreConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub wallet: Option<String>,
    pub chain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    pub rpc: Option<String>,
    pub chain_id: Option<u64>,
    pub network: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreConfig {
    pub path: Option<PathBuf>,
    pub encryption: Option<String>,
    pub kdf: Option<String>,
}

impl Config {
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let config_file = if let Some(path) = config_path {
            path.to_path_buf()
        } else {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".wallet")
                .join("config.toml")
        };

        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            toml::from_str(&content).map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, config_path: Option<&Path>) -> Result<()> {
        let config_file = if let Some(path) = config_path {
            path.to_path_buf()
        } else {
            let dir = dirs::home_dir().unwrap_or_default().join(".wallet");
            fs::create_dir_all(&dir)?;
            dir.join("config.toml")
        };

        let content = toml::to_string_pretty(self)?;
        fs::write(config_file, content)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut chains = HashMap::new();

        // EVM chains
        chains.insert(
            "ethereum".to_string(),
            ChainConfig {
                rpc: Some("https://eth.llamarpc.com".to_string()),
                chain_id: Some(1),
                network: None,
            },
        );

        chains.insert(
            "polygon".to_string(),
            ChainConfig {
                rpc: Some("https://polygon-rpc.com".to_string()),
                chain_id: Some(137),
                network: None,
            },
        );

        // Bitcoin
        chains.insert(
            "bitcoin".to_string(),
            ChainConfig {
                rpc: None,
                chain_id: None,
                network: Some("mainnet".to_string()),
            },
        );

        // Solana
        chains.insert(
            "solana".to_string(),
            ChainConfig {
                rpc: Some("https://api.mainnet-beta.solana.com".to_string()),
                chain_id: None,
                network: None,
            },
        );

        Config {
            default: DefaultConfig {
                wallet: None,
                chain: Some("ethereum".to_string()),
            },
            chains,
            keystore: KeystoreConfig {
                path: None,
                encryption: Some("aes-256-gcm".to_string()),
                kdf: Some("argon2id".to_string()),
            },
        }
    }
}
