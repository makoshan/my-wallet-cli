
use anyhow::Result;
use clap::Args;
use bip39::{Language, Mnemonic, Seed};
use crate::{keystore, output};

#[derive(Args)]
pub struct BalanceArgs {
    #[arg(short, long)] pub wallet: String,
    #[arg(short, long, default_value = "ethereum")] pub chain: String,
    #[arg(long)] pub rpc: Option<String>,
}

pub fn run(args: BalanceArgs, json: bool) -> Result<()> {
    let password = crate::keystore::read_password("Wallet password: ")?;
    let phrase = keystore::load_mnemonic(&args.wallet, &password)?;
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let seed = Seed::new(&mnemonic, "");
    let sb = seed.as_bytes();

    let address = match args.chain.as_str() {
        "ethereum"|"polygon"|"bsc"|"arbitrum"|"optimism"|"base"|"avalanche" => {
            tcx_evm::EvmKeystore::from_secret_bytes(&sb[..32])?.address()?.to_string()
        }
        "bitcoin" => {
            tcx_bitcoin_bdk::BitcoinKeystore::from_secret_bytes(&sb[..32], "bitcoin")?.address()?
        }
        "solana" => {
            tcx_solana::SolanaKeystore::from_secret_bytes(&sb[..32])?.address().to_string()
        }
        _ => anyhow::bail!("Unsupported chain: {}", args.chain),
    };

    #[derive(serde::Serialize)]
    struct Out { chain: String, address: String, balance: String, note: String }
    output::print_result(&Out {
        chain: args.chain, address,
        balance: "0".into(),
        note: args.rpc.unwrap_or_else(|| "No RPC configured".into()),
    }, json);
    Ok(())
}
