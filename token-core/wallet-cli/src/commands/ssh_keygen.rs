use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;
use tcx_ssh::mnemonic::MnemonicHandler;
use tcx_ssh::ssh_keys::SshKeyGenerator;
use std::path::PathBuf;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    wallet: Option<String>,
    mnemonic_phrase: Option<String>,
    output_path: Option<PathBuf>,
    comment: Option<String>,
    json_mode: bool,
) -> Result<()> {
    // 获取或生成助记词
    let mnemonic = if let Some(phrase) = mnemonic_phrase {
        // 使用提供的助记词
        MnemonicHandler::from_phrase(&phrase)?
    } else {
        // 从钱包生成助记词（模拟）
        let wallets = keystore_mgr.list_wallets();
        if wallets.is_empty() {
            output::print_error(json_mode, "No wallets found");
            return Ok(());
        }

        let _wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());
        
        // 在实际实现中，这里应该从 Keystore 中读取助记词
        // 现在我们生成一个新的
        MnemonicHandler::new(tcx_ssh::mnemonic::MnemonicLength::Words24)?
    };

    // 从助记词生成 SSH 密钥
    let ssh_keygen = SshKeyGenerator::from_mnemonic(
        &mnemonic,
        None,
        comment.as_deref(),
    )?;

    // 确定输出路径
    let key_path = output_path.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".ssh")
            .join("id_ed25519_mnemonic")
    });

    // 保存密钥对
    let (private_path, public_path) = ssh_keygen.save_to_files(&key_path)?;

    let result = json!({
        "status": "success",
        "private_key_path": private_path,
        "public_key_path": public_path,
        "public_key": ssh_keygen.public_key_openssh(),
        "fingerprint_md5": ssh_keygen.md5_fingerprint(),
        "fingerprint_sha256": ssh_keygen.sha256_fingerprint(),
        "key_type": "ssh-ed25519"
    });

    output::print_result(json_mode, "ssh_key", result);
    Ok(())
}
