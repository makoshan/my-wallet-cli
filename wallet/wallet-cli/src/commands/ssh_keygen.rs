
use anyhow::Result;
use clap::Args;
use bip39::{Language, Mnemonic, Seed};
use crate::{keystore, output};

#[derive(Args)]
pub struct SshKeygenArgs {
    #[arg(short, long)] pub wallet: String,
    #[arg(long, default_value = "wallet-ssh-key")] pub comment: String,
    #[arg(short, long)] pub output: Option<String>,
}

pub fn run(args: SshKeygenArgs, json: bool) -> Result<()> {
    let password = crate::keystore::read_password("Wallet password: ")?;
    let phrase = keystore::load_mnemonic(&args.wallet, &password)?;
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let seed = Seed::new(&mnemonic, "");
    let kp = tcx_ssh::SshKeypair::from_seed_bytes(seed.as_bytes())?;
    let pub_line = kp.authorized_keys_line(&args.comment);
    let fingerprint = kp.fingerprint_sha256();
    let private_pem = kp.private_key_openssh(&args.comment);

    if let Some(ref path) = args.output {
        std::fs::write(path, &private_pem)?;
        std::fs::write(format!("{}.pub", path), &pub_line)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        }
    }

    #[derive(serde::Serialize)]
    struct Out { public_key: String, fingerprint: String, output_file: Option<String> }
    output::print_result(&Out { public_key: pub_line, fingerprint, output_file: args.output }, json);
    Ok(())
}
