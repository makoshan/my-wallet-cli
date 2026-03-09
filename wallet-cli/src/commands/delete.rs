use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;

pub async fn handle(
    keystore_mgr: &mut KeystoreManager,
    wallet: Option<String>,
    force: bool,
    json_mode: bool,
) -> Result<()> {
    let wallets = keystore_mgr.list_wallets();
    if wallets.is_empty() {
        output::print_error(json_mode, "No wallets found");
        return Ok(());
    }

    let wallet_name = wallet.unwrap_or_else(|| wallets[0].name.clone());

    if !force {
        println!(
            "Are you sure you want to delete wallet '{}'? (yes/no)",
            wallet_name
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() != "yes" {
            output::print_success(json_mode, "Deletion cancelled");
            return Ok(());
        }
    }

    keystore_mgr.delete_wallet(&wallet_name)?;
    output::print_success(json_mode, &format!("Wallet '{}' deleted", wallet_name));
    Ok(())
}
