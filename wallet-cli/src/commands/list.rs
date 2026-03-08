use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(keystore_mgr: &KeystoreManager, json_mode: bool) -> Result<()> {
    let wallets = keystore_mgr.list_wallets();

    if wallets.is_empty() {
        output::print_success(json_mode, "No wallets found");
        return Ok(());
    }

    let wallet_list: Vec<_> = wallets
        .iter()
        .map(|w| {
            json!({
                "id": w.id,
                "name": w.name,
                "created_at": w.created_at
            })
        })
        .collect();

    let result = json!(wallet_list);
    output::print_result(json_mode, "wallets", result);
    Ok(())
}
