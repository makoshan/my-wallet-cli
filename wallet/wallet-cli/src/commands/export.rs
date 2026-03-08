
use anyhow::Result;
use clap::Args;
use crate::{keystore, output};

#[derive(Args)]
pub struct ExportArgs {
    #[arg(short, long)] pub wallet: String,
}

pub fn run(args: ExportArgs, json: bool) -> Result<()> {
    let password = crate::keystore::read_password("Wallet password: ")?;
    let mnemonic = keystore::load_mnemonic(&args.wallet, &password)?;
    #[derive(serde::Serialize)]
    struct Out { mnemonic: String }
    output::print_result(&Out { mnemonic }, json);
    Ok(())
}
