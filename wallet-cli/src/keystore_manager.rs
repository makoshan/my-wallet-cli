use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tcx_bitcoin_bdk::address::BitcoinAddress;
use tcx_constants::{coin_info_from_param, CurveType};
use tcx_evm::address::EvmAddress;
use tcx_keystore::{Keystore, Metadata};
use tcx_primitive::generate_mnemonic;
use tcx_solana::address::SolanaAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub keystore_file: String,
}

pub struct CreatedWallet {
    pub id: String,
    pub name: String,
    pub mnemonic: String,
    pub ethereum_address: String,
    pub solana_address: String,
    pub bitcoin_address: String,
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

    pub fn create_wallet(
        &mut self,
        name: String,
        password: &str,
        mnemonic: Option<String>,
    ) -> Result<CreatedWallet> {
        if self.metadata.values().any(|wallet| wallet.name == name) {
            return Err(anyhow::anyhow!("Wallet '{}' already exists", name));
        }

        let mnemonic = mnemonic
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(generate_mnemonic);

        let metadata = Metadata {
            name: name.clone(),
            identified_chain_types: Some(vec![
                "ETHEREUM".to_string(),
                "SOLANA".to_string(),
                "BITCOIN".to_string(),
            ]),
            ..Metadata::default()
        };

        let mut keystore = Keystore::from_mnemonic(&mnemonic, password, metadata)?;
        keystore.unlock_by_password(password)?;
        let ethereum_account = self.derive_evm_account(&mut keystore, "ethereum", 0)?;
        let solana_account = self.derive_solana_account(&mut keystore, 0)?;
        let bitcoin_account = self.derive_bitcoin_account(&mut keystore, 0)?;

        let id = keystore.id();
        let keystore_file = format!("{}.json", id);
        fs::write(self.keystore_dir.join(&keystore_file), keystore.to_json())?;

        let wallet_metadata = WalletMetadata {
            id: id.clone(),
            name: name.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            keystore_file,
        };

        self.metadata.insert(id.clone(), wallet_metadata);
        self.save_metadata()?;

        Ok(CreatedWallet {
            id,
            name,
            mnemonic,
            ethereum_address: ethereum_account.address,
            solana_address: solana_account.address,
            bitcoin_address: bitcoin_account.address,
        })
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

        let keystore_path = self
            .keystore_dir
            .join(&self.metadata[&wallet_id].keystore_file);
        if keystore_path.exists() {
            fs::remove_file(keystore_path)?;
        }

        self.metadata.remove(&wallet_id);
        self.save_metadata()?;

        Ok(())
    }

    pub fn derive_address(
        &self,
        name_or_id: &str,
        password: &str,
        chain: &str,
        index: u32,
    ) -> Result<String> {
        let wallet = self
            .get_wallet(name_or_id)
            .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;
        let mut keystore = self.load_keystore(&wallet)?;
        keystore.unlock_by_password(password)?;

        match chain.to_ascii_lowercase().as_str() {
            "ethereum" | "sepolia" | "ethereum_sepolia" | "polygon" | "arbitrum" | "optimism"
            | "base" | "avalanche" | "bsc" => Ok(self
                .derive_evm_account(&mut keystore, chain, index)?
                .address),
            "solana" | "solana_devnet" | "solana_testnet" => {
                Ok(self.derive_solana_account(&mut keystore, index)?.address)
            }
            "bitcoin" | "bitcoin_testnet" => Ok(self
                .derive_bitcoin_account_for_chain(&mut keystore, chain, index)?
                .address),
            other => Err(anyhow::anyhow!("Unsupported chain: {}", other)),
        }
    }

    fn save_metadata(&self) -> Result<()> {
        let metadata_file = self.keystore_dir.join("metadata.json");
        let content = serde_json::to_string_pretty(&self.metadata)?;
        fs::write(metadata_file, content)?;
        Ok(())
    }

    fn load_keystore(&self, wallet: &WalletMetadata) -> Result<Keystore> {
        let keystore_path = self.keystore_dir.join(&wallet.keystore_file);
        let content = fs::read_to_string(keystore_path)?;
        Keystore::from_json(&content)
    }

    fn derive_evm_account(
        &self,
        keystore: &mut Keystore,
        chain: &str,
        index: u32,
    ) -> Result<tcx_keystore::Account> {
        let network = match chain.to_ascii_lowercase().as_str() {
            "sepolia" | "ethereum_sepolia" => "TESTNET",
            _ => "MAINNET",
        };
        let mut coin_info =
            coin_info_from_param("ETHEREUM", network, "", CurveType::SECP256k1.as_str())?;
        coin_info.derivation_path = format!("m/44'/60'/0'/0/{}", index);
        keystore.derive_coin::<EvmAddress>(&coin_info)
    }

    fn derive_solana_account(
        &self,
        keystore: &mut Keystore,
        index: u32,
    ) -> Result<tcx_keystore::Account> {
        let coin_info = tcx_constants::CoinInfo {
            coin: "SOLANA".to_string(),
            derivation_path: format!("m/44'/501'/{}'/0'", index),
            curve: CurveType::ED25519,
            network: "MAINNET".to_string(),
            seg_wit: String::new(),
            chain_id: String::new(),
            contract_code: String::new(),
        };
        keystore.derive_coin::<SolanaAddress>(&coin_info)
    }

    fn derive_bitcoin_account(
        &self,
        keystore: &mut Keystore,
        index: u32,
    ) -> Result<tcx_keystore::Account> {
        self.derive_bitcoin_account_for_chain(keystore, "bitcoin", index)
    }

    fn derive_bitcoin_account_for_chain(
        &self,
        keystore: &mut Keystore,
        chain: &str,
        index: u32,
    ) -> Result<tcx_keystore::Account> {
        let network = match chain.to_ascii_lowercase().as_str() {
            "bitcoin_testnet" => "TESTNET",
            _ => "MAINNET",
        };
        let mut coin_info = coin_info_from_param(
            "BITCOIN",
            network,
            "VERSION_0",
            CurveType::SECP256k1.as_str(),
        )?;
        coin_info.derivation_path = match network {
            "MAINNET" => format!("m/84'/0'/0'/0/{}", index),
            _ => format!("m/84'/1'/0'/0/{}", index),
        };
        keystore.derive_coin::<BitcoinAddress>(&coin_info)
    }
}

#[cfg(test)]
mod tests {
    use super::KeystoreManager;
    use tempfile::TempDir;

    #[test]
    fn create_wallet_derives_default_eth_solana_and_bitcoin_addresses() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = KeystoreManager::new(temp_dir.path().to_path_buf()).unwrap();

        let wallet = manager
            .create_wallet(
                "mako1".to_string(),
                "test-password",
                Some(
                    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
                        .to_string(),
                ),
            )
            .unwrap();

        assert!(wallet.ethereum_address.starts_with("0x"));
        assert!(!wallet.solana_address.is_empty());
        assert!(wallet.bitcoin_address.starts_with("bc1"));
    }
}
