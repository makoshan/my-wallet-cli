
mod commands;
mod keystore;
mod config;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wallet", about = "Multi-chain wallet CLI", version = "0.1.0")]
struct Cli {
    #[arg(long, default_value = "text")]
    output: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet
    Create(commands::create::CreateArgs),
    /// List all wallets
    List,
    /// Show wallet address for a chain
    Address(commands::address::AddressArgs),
    /// Query wallet balance
    Balance(commands::balance::BalanceArgs),
    /// Sign a message
    SignMessage(commands::sign_message::SignMessageArgs),
    /// Send a transaction
    Send(commands::send::SendArgs),
    /// Generate SSH key from wallet mnemonic
    SshKeygen(commands::ssh_keygen::SshKeygenArgs),
    /// Export wallet mnemonic
    Export(commands::export::ExportArgs),
    /// Delete a wallet
    Delete(commands::delete::DeleteArgs),
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    let json = cli.output == "json";
    match cli.command {
        Commands::Create(a)     => commands::create::run(a, json),
        Commands::List          => commands::list::run(json),
        Commands::Address(a)    => commands::address::run(a, json),
        Commands::Balance(a)    => commands::balance::run(a, json),
        Commands::SignMessage(a)=> commands::sign_message::run(a, json),
        Commands::Send(a)       => commands::send::run(a, json),
        Commands::SshKeygen(a)  => commands::ssh_keygen::run(a, json),
        Commands::Export(a)     => commands::export::run(a, json),
        Commands::Delete(a)     => commands::delete::run(a, json),
    }
}
