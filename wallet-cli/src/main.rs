use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use wallet_cli::{commands, config, keystore_manager};

#[derive(Parser)]
#[clap(name = "wallet")]
#[clap(about = "Multi-chain cryptocurrency wallet CLI", long_about = None)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(global = true, long, help = "Path to config file")]
    config: Option<PathBuf>,

    #[clap(global = true, long, help = "Path to keystore directory")]
    keystore: Option<PathBuf>,

    #[clap(global = true, long, help = "Verbose output")]
    verbose: bool,

    #[clap(global = true, long, help = "JSON output format")]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet
    Create {
        #[clap(long, help = "Wallet name")]
        name: Option<String>,

        #[clap(long, help = "Wallet password")]
        password: Option<String>,

        #[clap(long, help = "Import existing mnemonic")]
        mnemonic: Option<String>,
    },

    /// List all wallets
    List,

    /// Show wallet address
    Address {
        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Wallet password")]
        password: Option<String>,

        #[clap(long, help = "Chain name (ethereum, bitcoin, solana, etc.)")]
        chain: Option<String>,

        #[clap(long, help = "Account index")]
        index: Option<u32>,
    },

    /// Query wallet balance
    Balance {
        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Chain name")]
        chain: Option<String>,

        #[clap(long, help = "RPC URL")]
        rpc: Option<String>,
    },

    /// Sign a message
    SignMessage {
        #[clap(help = "Message to sign")]
        message: String,

        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Chain name")]
        chain: Option<String>,

        #[clap(long, help = "Account index")]
        index: Option<u32>,
    },

    /// Send a transaction
    Send {
        #[clap(help = "Amount to send")]
        amount: String,

        #[clap(long, help = "Recipient address")]
        to: String,

        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Chain name")]
        chain: Option<String>,

        #[clap(long, help = "Gas price")]
        gas_price: Option<String>,

        #[clap(long, help = "Gas limit")]
        gas_limit: Option<u64>,
    },

    /// Export wallet
    Export {
        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Export format (json, mnemonic)")]
        format: Option<String>,

        #[clap(long, help = "Export password")]
        password: Option<String>,
    },

    /// Delete wallet
    Delete {
        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Skip confirmation")]
        force: bool,
    },

    /// Generate SSH key from mnemonic
    SshKeygen {
        #[clap(long, help = "Wallet name")]
        wallet: Option<String>,

        #[clap(long, help = "Mnemonic phrase")]
        mnemonic: Option<String>,

        #[clap(long, help = "Output path for SSH key")]
        output: Option<std::path::PathBuf>,

        #[clap(long, help = "SSH key comment")]
        comment: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    // Initialize config
    let config = config::Config::load(cli.config.as_deref())?;

    // Initialize keystore manager
    let keystore_path = cli.keystore.clone().unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".wallet")
            .join("keystore")
    });
    let mut keystore_mgr = keystore_manager::KeystoreManager::new(keystore_path)?;

    // Execute commands
    match cli.command {
        Commands::Create {
            name,
            password,
            mnemonic,
        } => {
            commands::create::handle(&mut keystore_mgr, name, password, mnemonic, cli.json).await?;
        }
        Commands::List => {
            commands::list::handle(&keystore_mgr, cli.json).await?;
        }
        Commands::Address {
            wallet,
            password,
            chain,
            index,
        } => {
            commands::address::handle(&keystore_mgr, wallet, password, chain, index, cli.json)
                .await?;
        }
        Commands::Balance { wallet, chain, rpc } => {
            commands::balance::handle(&keystore_mgr, &config, wallet, chain, rpc, cli.json).await?;
        }
        Commands::SignMessage {
            message,
            wallet,
            chain,
            index,
        } => {
            commands::sign_message::handle(&keystore_mgr, message, wallet, chain, index, cli.json)
                .await?;
        }
        Commands::Send {
            amount,
            to,
            wallet,
            chain,
            gas_price,
            gas_limit,
        } => {
            commands::send::handle(
                &keystore_mgr,
                &config,
                amount,
                to,
                wallet,
                chain,
                gas_price,
                gas_limit,
                cli.json,
            )
            .await?;
        }
        Commands::Export {
            wallet,
            format,
            password,
        } => {
            commands::export::handle(&keystore_mgr, wallet, format, password, cli.json).await?;
        }
        Commands::Delete { wallet, force } => {
            commands::delete::handle(&mut keystore_mgr, wallet, force, cli.json).await?;
        }
        Commands::SshKeygen {
            wallet,
            mnemonic,
            output,
            comment,
        } => {
            commands::ssh_keygen::handle(
                &keystore_mgr,
                wallet,
                mnemonic,
                output,
                comment,
                cli.json,
            )
            .await?;
        }
    }

    Ok(())
}
