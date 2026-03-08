# My Wallet CLI

[![Rust CI](https://github.com/makoshan/my-wallet-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/makoshan/my-wallet-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/wallet-cli.svg)](https://crates.io/crates/wallet-cli)

A powerful, secure, and extensible multi-chain wallet command-line interface, built in Rust.

This project is a refactored and enhanced version based on the original `token-core-monorepo`. It removes all hardware wallet (imKey) dependencies, flattens the project structure for simplicity, and adds modern chain support using best-in-class Rust libraries.

## ✨ Features

- **Multi-Chain Support**: Manages wallets for numerous blockchains.
- **Modern Libraries**: 
    - **EVM Chains**: Powered by [Alloy](https://github.com/alloy-rs/alloy).
    - **Bitcoin**: Powered by [BDK](https://bitcoindevkit.org/).
    - **Solana**: Powered by the official [Solana SDK](https://github.com/solana-labs/solana/tree/master/sdk).
- **Secure Keystore**: Encrypted, password-protected keystore using AES-256-GCM and Argon2id.
- **SSH Key Generation**: Generate Ed25519 SSH keys from your mnemonic phrase.
- **Extensible Architecture**: Clean, modular design makes it easy to add support for new chains.
- **Comprehensive CLI**: A user-friendly command-line interface for all wallet operations.

## 🚀 Quick Start

For a detailed guide, see [**QUICKSTART.md**](./QUICKSTART.md).

### 1. Installation

Ensure you have Rust `1.70+` installed.

```bash
# Clone the repository
git clone https://github.com/makoshan/my-wallet-cli.git
cd my-wallet-cli

# Install the CLI binary
cargo install --path wallet-cli
```

### 2. Create a Wallet

```bash
# Create a new wallet named 'my_first_wallet'
wallet create --name my_first_wallet

# You will be prompted to set a password.
# Your mnemonic phrase will be displayed. SAVE IT SECURELY!
```

### 3. Get an Address

```bash
# Get your Ethereum address
wallet address --chain ethereum --wallet my_first_wallet

# Get your Bitcoin address
wallet address --chain bitcoin --wallet my_first_wallet

# Get your Solana address
wallet address --chain solana --wallet my_first_wallet
```

### 4. Generate SSH Key

```bash
# Generate an SSH key from your wallet's mnemonic
wallet ssh-keygen --wallet my_first_wallet --output ~/.ssh/id_wallet

# The public key will be saved to ~/.ssh/id_wallet.pub
```

## 🏗️ Project Structure

This repository is a Cargo workspace with a flattened structure. All crates (`tcx-*`, `wallet-cli`, etc.) are located in the root directory.

| Crate | Description |
| :--- | :--- |
| `wallet-cli` | The main CLI application binary. |
| `tcx` | The core library facade, exporting a unified API (originally for protobuf). |
| `tcx-keystore` | Manages encrypted wallet storage. |
| `tcx-crypto` | Core cryptographic primitives. |
| `tcx-primitive` | Shared data structures like `Bip39`, `PrivateKey`, etc. |
| `tcx-evm` | **New**: EVM chain support using Alloy. |
| `tcx-bitcoin-bdk` | **New**: Bitcoin support using BDK. |
| `tcx-solana` | **New**: Solana support using the Solana SDK. |
| `tcx-ssh` | **New**: SSH key generation from mnemonics. |
| `tcx-*` | Support for various other chains inherited from token-core. |

For a deep dive into the architecture, see [**ARCHITECTURE.md**](./ARCHITECTURE.md).

## 🔧 Development

### Build

```bash
# Build the entire workspace in debug mode
cargo build

# Build the CLI in release mode
cargo build --release -p wallet-cli
```

### Test

```bash
# Run all tests across the workspace
cargo test --all
```

### Code Quality

```bash
# Check formatting
cargo fmt --all -- --check

# Run Clippy linter
cargo clippy --all-targets -- -D warnings
```

## 🤝 Contributing

Contributions are welcome! Please read the [**CONTRIBUTING.md**](./CONTRIBUTING.md) guide for details on our code of conduct and the process for submitting pull requests.

## 🔐 Security

Security is a top priority. Please see the [**SECURITY.md**](./SECURITY.md) file for details on our security policy and how to report vulnerabilities.

## 📄 License

This project is licensed under the MIT License - see the [**LICENSE**](./LICENSE) file for details.

## 🙏 Acknowledgements

- This project is heavily based on the great work done by the `imToken` team on [token-core-monorepo](https://github.com/consenlabs/token-core-monorepo).
- The SSH key generation feature is inspired by [mnemossh](https://github.com/abkvme/mnemossh).
