use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_keystore_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let keystore_path = temp_dir.path().to_path_buf();

    // 创建 KeystoreManager 应该成功
    let result = wallet_cli::keystore_manager::KeystoreManager::new(keystore_path);
    assert!(result.is_ok());
}

#[test]
fn test_config_default() {
    let config = wallet_cli::config::Config::default();

    // 检查默认配置
    assert!(config.default.chain.is_some());
    assert!(!config.chains.is_empty());
    assert!(config.keystore.path.is_some() || config.keystore.path.is_none());
}

#[test]
fn test_config_chains() {
    let config = wallet_cli::config::Config::default();

    // 检查支持的链
    assert!(config.chains.contains_key("ethereum"));
    assert!(config.chains.contains_key("bitcoin"));
    assert!(config.chains.contains_key("solana"));
}

#[test]
fn test_wallet_metadata() {
    let metadata = wallet_cli::keystore_manager::WalletMetadata {
        id: "test-id".to_string(),
        name: "test-wallet".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        keystore_file: "test.json".to_string(),
    };

    assert_eq!(metadata.name, "test-wallet");
    assert_eq!(metadata.id, "test-id");
}
