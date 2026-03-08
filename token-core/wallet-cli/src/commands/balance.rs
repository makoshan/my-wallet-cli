use crate::config::Config;
use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    config: &Config,
    wallet: Option<String>,
    chain: Option<String>,
    _rpc: Option<String>,
    json_mode: bool,
) -> Result<()> {
    let wallets = keystore_mgr.list_wallets();
    if wallets.is_empty() {
        output::print_error(json_mode, "No wallets found");
        return Ok(());
    }

    let wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());
    let chain_name = chain.unwrap_or_else(|| {
        config
            .default
            .chain
            .clone()
            .unwrap_or_else(|| "ethereum".to_string())
    });

    // Mock balance - in real implementation, this would query RPC
    let mock_balance = "1.5";

    let result = json!({
        "balance": mock_balance,
        "chain": chain_name,
        "wallet": wallet_name,
        "unit": "ETH"
    });

    output::print_result(json_mode, "balance", result);
    Ok(())
}
