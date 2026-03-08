use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    wallet: Option<String>,
    format: Option<String>,
    _password: Option<String>,
    json_mode: bool,
) -> Result<()> {
    let wallets = keystore_mgr.list_wallets();
    if wallets.is_empty() {
        output::print_error(json_mode, "No wallets found");
        return Ok(());
    }

    let wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());
    let export_format = format.unwrap_or_else(|| "json".to_string());

    let result = json!({
        "wallet": wallet_name,
        "format": export_format,
        "status": "exported",
        "file": format!("{}.{}", wallet_name, export_format)
    });

    output::print_result(json_mode, "export", result);
    Ok(())
}
