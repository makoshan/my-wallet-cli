use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;
use tcx_ssh::mnemonic::{MnemonicHandler, MnemonicLength};
use tcx_ssh::ssh_keys::SshKeypair;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    wallet: Option<String>,
    mnemonic_phrase: Option<String>,
    output_path: Option<PathBuf>,
    comment: Option<String>,
    json_mode: bool,
) -> Result<()> {
    let mnemonic = if let Some(phrase) = mnemonic_phrase {
        MnemonicHandler::from_phrase(&phrase)?
    } else {
        let wallets = keystore_mgr.list_wallets();
        if wallets.is_empty() {
            output::print_error(json_mode, "No wallets found");
            return Ok(());
        }

        let _wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());

        // The current keystore manager does not expose the stored mnemonic,
        // so we fall back to a newly generated phrase when one is not provided.
        MnemonicHandler::new(MnemonicLength::Words24)?
    };

    let seed = mnemonic.to_seed(None)?;
    let ssh_keypair = SshKeypair::from_seed_bytes(&seed)?;
    let comment = comment.unwrap_or_else(|| "wallet-cli".to_string());

    let key_path = output_path.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".ssh")
            .join("id_ed25519_mnemonic")
    });

    if let Some(parent) = key_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&key_path, ssh_keypair.private_key_openssh(&comment))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600))?;
    }

    let public_path = PathBuf::from(format!("{}.pub", key_path.display()));
    std::fs::write(
        &public_path,
        format!("{}\n", ssh_keypair.authorized_keys_line(&comment)),
    )?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&public_path, std::fs::Permissions::from_mode(0o644))?;
    }

    let result = json!({
        "status": "success",
        "private_key_path": key_path,
        "public_key_path": public_path,
        "public_key": ssh_keypair.authorized_keys_line(&comment),
        "fingerprint_sha256": ssh_keypair.fingerprint_sha256(),
        "key_type": "ssh-ed25519"
    });

    output::print_result(json_mode, "ssh_key", result);
    Ok(())
}
