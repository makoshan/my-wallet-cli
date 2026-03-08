use crate::config::Config;
use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;

pub async fn handle(
    keystore_mgr: &KeystoreManager,
    _config: &Config,
    amount: String,
    to: String,
    wallet: Option<String>,
    chain: Option<String>,
    _gas_price: Option<String>,
    _gas_limit: Option<u64>,
    json_mode: bool,
) -> Result<()> {
    let wallets = keystore_mgr.list_wallets();
    if wallets.is_empty() {
        output::print_error(json_mode, "No wallets found");
        return Ok(());
    }

    let wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());
    let chain_name = chain.unwrap_or_else(|| "ethereum".to_string());

    // Mock transaction hash - in real implementation, this would sign and send
    let mock_tx_hash = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

    let result = json!({
        "tx_hash": mock_tx_hash,
        "amount": amount,
        "to": to,
        "chain": chain_name,
        "wallet": wallet_name,
        "status": "pending"
    });

    output::print_result(json_mode, "transaction", result);
    Ok(())
}
