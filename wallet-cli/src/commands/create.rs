use crate::keystore_manager::KeystoreManager;
use crate::output;
use anyhow::Result;
use serde_json::json;
use std::io::{self, IsTerminal, Write};
use uuid::Uuid;

pub async fn handle(
    keystore_mgr: &mut KeystoreManager,
    name: Option<String>,
    password: Option<String>,
    mnemonic: Option<String>,
    json_mode: bool,
) -> Result<()> {
    let default_name = default_wallet_name();
    let wallet_name = match name {
        Some(name) => name,
        None if io::stdin().is_terminal() => prompt_wallet_name(&default_name)?,
        None => default_name,
    };
    let password = password.unwrap_or_else(|| {
        rpassword::prompt_password("Choose a wallet password: ").unwrap_or_default()
    });

    let wallet = keystore_mgr.create_wallet(wallet_name.clone(), &password, mnemonic)?;

    print_created_wallet(json_mode, &wallet);
    Ok(())
}

fn prompt_wallet_name(default_name: &str) -> Result<String> {
    print!("Wallet name [{}]: ", default_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let name = input.trim();

    if name.is_empty() {
        Ok(default_name.to_string())
    } else {
        Ok(name.to_string())
    }
}

fn default_wallet_name() -> String {
    let suffix = Uuid::new_v4().simple().to_string();
    format!("wallet-{}", &suffix[..8])
}

fn print_created_wallet(json_mode: bool, wallet: &crate::keystore_manager::CreatedWallet) {
    if json_mode {
        let result = json!({
            "wallet_id": wallet.id,
            "wallet_name": wallet.name,
            "mnemonic": wallet.mnemonic,
            "ethereum_address": wallet.ethereum_address,
            "solana_address": wallet.solana_address,
            "bitcoin_address": wallet.bitcoin_address,
            "status": "created"
        });
        output::print_result(true, "wallet", result);
        return;
    }

    println!("Wallet created successfully.");
    println!();
    println!("Name: {}", wallet.name);
    println!("Wallet ID: {}", wallet.id);
    println!("Ethereum address: {}", wallet.ethereum_address);
    println!("Solana address: {}", wallet.solana_address);
    println!("Bitcoin address: {}", wallet.bitcoin_address);
    println!("Recovery phrase:");
    println!("  {}", wallet.mnemonic);
    println!();
    println!("Keep this recovery phrase somewhere safe. Anyone with it can restore this wallet.");
}

#[cfg(test)]
mod tests {
    use super::default_wallet_name;

    #[test]
    fn default_wallet_name_looks_random() {
        let first = default_wallet_name();
        let second = default_wallet_name();

        assert!(first.starts_with("wallet-"));
        assert!(second.starts_with("wallet-"));
        assert_ne!(first, second);
    }
}
