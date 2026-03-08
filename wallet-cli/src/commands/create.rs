use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(
    keystore_mgr: &mut KeystoreManager,
    name: Option<String>,
    password: Option<String>,
    _mnemonic: Option<String>,
    json_mode: bool,
) -> Result<()> {
    let wallet_name = name.unwrap_or_else(|| format!("wallet_{}", uuid::Uuid::new_v4()));
    let _password = password.unwrap_or_else(|| {
        rpassword::prompt_password("Enter wallet password: ").unwrap_or_default()
    });

    // Create wallet
    let wallet_id = keystore_mgr.create_wallet(wallet_name.clone(), None)?;

    let result = json!({
        "wallet_id": wallet_id,
        "wallet_name": wallet_name,
        "status": "created"
    });

    output::print_result(json_mode, "wallet", result);
    Ok(())
}
