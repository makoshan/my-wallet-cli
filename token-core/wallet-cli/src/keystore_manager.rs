use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub keystore_file: String,
}

pub struct KeystoreManager {
    keystore_dir: PathBuf,
    metadata: HashMap<String, WalletMetadata>,
}

impl KeystoreManager {
    pub fn new(keystore_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&keystore_dir)?;

        let mut metadata = HashMap::new();
        let metadata_file = keystore_dir.join("metadata.json");

        if metadata_file.exists() {
            let content = fs::read_to_string(&metadata_file)?;
            metadata = serde_json::from_str(&content)?;
        }

        Ok(KeystoreManager {
            keystore_dir,
            metadata,
        })
    }

    pub fn create_wallet(&mut self, name: String, mnemonic: Option<String>) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let keystore_file = format!("{}.json", id);

        let metadata = WalletMetadata {
            id: id.clone(),
            name,
            created_at: chrono::Utc::now().to_rfc3339(),
            keystore_file,
        };

        self.metadata.insert(id.clone(), metadata);
        self.save_metadata()?;

        Ok(id)
    }

    pub fn list_wallets(&self) -> Vec<WalletMetadata> {
        self.metadata.values().cloned().collect()
    }

    pub fn get_wallet(&self, name_or_id: &str) -> Option<WalletMetadata> {
        // Try to find by ID first
        if let Some(wallet) = self.metadata.get(name_or_id) {
            return Some(wallet.clone());
        }

        // Try to find by name
        self.metadata
            .values()
            .find(|w| w.name == name_or_id)
            .cloned()
    }

    pub fn delete_wallet(&mut self, name_or_id: &str) -> Result<()> {
        let wallet_id = if let Some(wallet) = self.get_wallet(name_or_id) {
            wallet.id
        } else {
            return Err(anyhow::anyhow!("Wallet not found"));
        };

        let keystore_path = self.keystore_dir.join(&self.metadata[&wallet_id].keystore_file);
        if keystore_path.exists() {
            fs::remove_file(keystore_path)?;
        }

        self.metadata.remove(&wallet_id);
        self.save_metadata()?;

        Ok(())
    }

    pub fn get_keystore_path(&self, wallet_id: &str) -> Result<PathBuf> {
        let wallet = self
            .metadata
            .get(wallet_id)
            .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

        Ok(self.keystore_dir.join(&wallet.keystore_file))
    }

    fn save_metadata(&self) -> Result<()> {
        let metadata_file = self.keystore_dir.join("metadata.json");
        let content = serde_json::to_string_pretty(&self.metadata)?;
        fs::write(metadata_file, content)?;
        Ok(())
    }
}
