
use anyhow::Result;
use clap::Args;
use bip39::{Language, Mnemonic, Seed};
use crate::{keystore, output};

#[derive(Args)]
pub struct SignMessageArgs {
    #[arg(short, long)] pub wallet: String,
    #[arg(short, long, default_value = "ethereum")] pub chain: String,
    pub message: String,
}

pub fn run(args: SignMessageArgs, json: bool) -> Result<()> {
    let password = crate::keystore::read_password("Wallet password: ")?;
    let phrase = keystore::load_mnemonic(&args.wallet, &password)?;
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let seed = Seed::new(&mnemonic, "");
    let sb = seed.as_bytes();

    match args.chain.as_str() {
        "ethereum"|"polygon"|"bsc"|"arbitrum"|"optimism"|"base"|"avalanche" => {
            let ks = tcx_evm::EvmKeystore::from_secret_bytes(&sb[..32])?;
            let out = ks.sign_message(&tcx_evm::transaction::EvmMessageInput { message: args.message })?;
            output::print_result(&out, json);
        }
        "bitcoin" => {
            let ks = tcx_bitcoin_bdk::BitcoinKeystore::from_secret_bytes(&sb[..32], "bitcoin")?;
            let out = ks.sign_message(&tcx_bitcoin_bdk::transaction::BitcoinMessageInput { message: args.message })?;
            output::print_result(&out, json);
        }
        "solana" => {
            let ks = tcx_solana::SolanaKeystore::from_secret_bytes(&sb[..32])?;
            let out = ks.sign_message(&tcx_solana::transaction::SolanaMessageInput { message: args.message })?;
            output::print_result(&out, json);
        }
        _ => anyhow::bail!("Unsupported chain: {}", args.chain),
    }
    Ok(())
}
