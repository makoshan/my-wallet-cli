use anyhow::Result;
use clap::Args;
use bip39::{Language, Mnemonic, MnemonicType};
use crate::{config, keystore, output};

#[derive(Args)]
pub struct CreateArgs {
    #[arg(short, long)] pub name: String,
    #[arg(long, default_value = "24")] pub words: u32,
}

pub fn run(args: CreateArgs, json: bool) -> Result<()> {
    config::ensure_dirs()?;
    let mtype = if args.words == 12 { MnemonicType::Words12 } else { MnemonicType::Words24 };
    let mnemonic = Mnemonic::new(mtype, Language::English);
    let phrase = mnemonic.phrase().to_string();

    let password = crate::keystore::read_password("Set wallet password: ")?;

    // In non-interactive mode (pipe/CI), skip confirmation
    if atty::is(atty::Stream::Stdin) {
        let confirm = crate::keystore::read_password("Confirm password: ")?;
        if password != confirm {
            anyhow::bail!("Passwords do not match");
        }
    }

    let id = keystore::save_wallet(&args.name, &phrase, &password)?;

    #[derive(serde::Serialize)]
    struct Out { id: String, name: String, mnemonic: String }
    output::print_result(&Out { id, name: args.name, mnemonic: phrase }, json);
    Ok(())
}
