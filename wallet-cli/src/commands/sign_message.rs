use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    message: String,
    wallet: Option<String>,
    chain: Option<String>,
    index: Option<u32>,
    json_mode: bool,
) -> Result<()> {
    let wallets = keystore_mgr.list_wallets();
    if wallets.is_empty() {
        output::print_error(json_mode, "No wallets found");
        return Ok(());
    }

    let wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());
    let chain_name = chain.unwrap_or_else(|| "ethereum".to_string());
    let account_index = index.unwrap_or(0);

    // Mock signature - in real implementation, this would use tcx
    let mock_signature = format!("0x{}", hex::encode(message.as_bytes()));

    let result = json!({
        "signature": mock_signature,
        "message": message,
        "chain": chain_name,
        "wallet": wallet_name,
        "index": account_index
    });

    output::print_result(json_mode, "signature", result);
    Ok(())
}
