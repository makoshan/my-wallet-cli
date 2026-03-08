
use anyhow::Result;
use clap::Args;
use crate::output;

#[derive(Args)]
pub struct SendArgs {
    #[arg(short, long)] pub wallet: String,
    #[arg(short, long, default_value = "ethereum")] pub chain: String,
    #[arg(long)] pub to: String,
    #[arg(long)] pub amount: String,
    #[arg(long)] pub rpc: Option<String>,
}

pub fn run(args: SendArgs, json: bool) -> Result<()> {
    output::print_text(
        &format!("[dry-run] Send {} to {} on {} (RPC broadcast not yet implemented)",
            args.amount, args.to, args.chain),
        json,
    );
    Ok(())
}
