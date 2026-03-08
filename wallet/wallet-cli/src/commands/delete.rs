
use anyhow::Result;
use clap::Args;
use crate::{keystore, output};

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(short, long)] pub wallet: String,
}

pub fn run(args: DeleteArgs, json: bool) -> Result<()> {
    let confirm = crate::keystore::read_password("Wallet password (to confirm deletion): ")?;
    keystore::load_mnemonic(&args.wallet, &confirm)?;
    keystore::delete_wallet(&args.wallet)?;
    output::print_text(&format!("Wallet '{}' deleted", args.wallet), json);
    Ok(())
}
