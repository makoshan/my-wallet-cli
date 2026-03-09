use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    wallet: Option<String>,
    password: Option<String>,
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
    let password = password.unwrap_or_else(|| {
        rpassword::prompt_password("Enter wallet password: ").unwrap_or_default()
    });
    let chain_name = chain.unwrap_or_else(|| "ethereum".to_string());
    let account_index = index.unwrap_or(0);
    let address =
        keystore_mgr.derive_address(&wallet_name, &password, &chain_name, account_index)?;

    let result = json!({
        "address": address,
        "chain": chain_name,
        "wallet": wallet_name,
        "index": account_index
    });

    output::print_result(json_mode, "address", result);
    Ok(())
}
