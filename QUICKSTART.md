# Quick Start Guide

This guide will walk you through installing `my-wallet-cli` and using its main features in 5 minutes.

## 1. Prerequisites

- **Rust**: You need Rust `1.70` or later. Install from [rustup.rs](https://rustup.rs/).
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Build Tools**: A C compiler is needed for some dependencies.
  ```bash
  # On Debian/Ubuntu
  sudo apt-get update && sudo apt-get install build-essential
  # On macOS
  xcode-select --install
  ```

## 2. Installation

```bash
# Clone the repository
git clone https://github.com/makoshan/my-wallet-cli.git
cd my-wallet-cli

# Install the binary
cargo install --path wallet-cli

# Verify installation
wallet --version
```

## 3. Your First Wallet

```bash
wallet create --name my_main_wallet
```

You will be prompted to set a password. The tool will then display your **12-word mnemonic phrase**.

> **⚠️ CRITICAL**: Write down your mnemonic phrase and store it securely offline. Anyone with this phrase can access your funds. It will only be shown once.

## 4. Core Commands

### List Wallets
```bash
wallet list
```

### Get Addresses
```bash
# Ethereum
wallet address --chain ethereum --wallet my_main_wallet

# Bitcoin (SegWit)
wallet address --chain bitcoin --wallet my_main_wallet

# Solana
wallet address --chain solana --wallet my_main_wallet
```

### Check Balances
```bash
wallet balance --chain ethereum --wallet my_main_wallet
wallet balance --chain bitcoin --wallet my_main_wallet
```

### Sign a Message
```bash
wallet sign-message "Hello, world!" --chain ethereum --wallet my_main_wallet
```

### Send a Transaction
```bash
# Send ETH on Sepolia testnet
wallet send 0.01 ETH --to 0xRecipient... --chain sepolia --wallet my_main_wallet

# Send BTC on testnet
wallet send 0.001 BTC --to tb1q... --chain bitcoin_testnet --wallet my_main_wallet
```

## 5. SSH Key Generation

Generate an Ed25519 SSH keypair from your wallet's mnemonic.

```bash
wallet ssh-keygen --wallet my_main_wallet --output ~/.ssh/id_wallet_main
```

- Private key → `~/.ssh/id_wallet_main`
- Public key → `~/.ssh/id_wallet_main.pub`

```bash
# Add to a server
cat ~/.ssh/id_wallet_main.pub >> ~/.ssh/authorized_keys
```

## 6. Wallet Management

```bash
# Export mnemonic (use with extreme caution)
wallet export --wallet my_main_wallet

# Delete a wallet (irreversible)
wallet delete --wallet my_main_wallet
```

## 7. Configuration

Edit `~/.wallet/config.toml` to add API keys for balance lookups:

```toml
[api_keys]
etherscan = "YOUR_ETHERSCAN_API_KEY"
```

For more details, see [ARCHITECTURE.md](./ARCHITECTURE.md).
