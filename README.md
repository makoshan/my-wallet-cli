# My Wallet CLI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/wallet-cli.svg)](https://crates.io/crates/wallet-cli)

`my-wallet-cli` is a Rust workspace centered around a multi-chain wallet CLI and a set of reusable chain/keystore crates.

This project is a refactored fork of imToken's [token-core-monorepo](https://github.com/consenlabs/token-core-monorepo). It flattens the repository structure, keeps the software-wallet keystore flow, and adds a simpler CLI-oriented workspace layout.

## Overview

Today, the repository is strongest in these areas:

- Wallet creation and mnemonic import.
- Encrypted keystore storage backed by `tcx-keystore`.
- Real address derivation for EVM, Bitcoin, and Solana accounts.
- SSH Ed25519 key generation from a mnemonic phrase.
- A Cargo workspace layout that makes the individual crates easier to build and test.

Some CLI commands are still placeholders and should be treated as scaffolding rather than production wallet operations.

## Current CLI Status

Implemented commands:

- `wallet create`: creates a new wallet or imports an existing mnemonic, then prints the wallet id, recovery phrase, and default Ethereum, Solana, and Bitcoin addresses.
- `wallet list`: lists stored wallets from local metadata.
- `wallet address`: derives a real address from the encrypted keystore after unlocking with the wallet password.
- `wallet delete`: deletes a wallet and its keystore file.
- `wallet ssh-keygen`: writes an Ed25519 SSH keypair derived from a mnemonic.

Placeholder commands:

- `wallet balance`: currently returns mock balance data.
- `wallet sign-message`: currently returns a mock signature.
- `wallet send`: currently returns a mock transaction hash.
- `wallet export`: currently returns a mock exported filename.

## Supported Address Derivation

The current `wallet address` implementation supports:

- EVM-style chains: `ethereum`, `sepolia`, `ethereum_sepolia`, `polygon`, `arbitrum`, `optimism`, `base`, `avalanche`, `bsc`
- Solana-style chains: `solana`, `solana_devnet`, `solana_testnet`
- Bitcoin-style chains: `bitcoin`, `bitcoin_testnet`

Default derivation behavior in the current code:

- Ethereum-family chains use `m/44'/60'/0'/0/{index}`
- Solana uses `m/44'/501'/{index}'/0'`
- Bitcoin mainnet uses `m/84'/0'/0'/0/{index}`
- Bitcoin testnet uses `m/84'/1'/0'/0/{index}`

## Quick Start

For a longer walkthrough, see [QUICKSTART.md](./QUICKSTART.md).

### 1. Install

Make sure Rust `1.70+` is installed.

```bash
git clone https://github.com/makoshan/my-wallet-cli.git
cd my-wallet-cli
cargo install --path wallet-cli
```

### 2. Create a wallet

```bash
wallet create --name my-wallet
```

Behavior in the current implementation:

- If `--name` is omitted in an interactive terminal, the CLI prompts for a name and offers a generated default like `wallet-1a2b3c4d`.
- If `--password` is omitted, the CLI prompts for one.
- The command prints the wallet id, recovery phrase, and default Ethereum, Solana, and Bitcoin addresses.

To import an existing mnemonic:

```bash
wallet create \
  --name imported-wallet \
  --password test-password \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

### 3. List wallets

```bash
wallet list
```

### 4. Derive addresses

The address command unlocks the keystore with the wallet password and derives the requested address.

```bash
wallet address --wallet my-wallet --chain ethereum
wallet address --wallet my-wallet --chain bitcoin --index 0
wallet address --wallet my-wallet --chain solana --index 0
```

If `--password` is omitted, the CLI prompts for it.

You can also script it:

```bash
wallet address \
  --wallet my-wallet \
  --password test-password \
  --chain sepolia \
  --index 1 \
  --json
```

### 5. Generate an SSH key from a mnemonic

Recommended usage:

```bash
wallet ssh-keygen \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" \
  --output ~/.ssh/id_wallet \
  --comment my-wallet
```

This writes:

- private key: `~/.ssh/id_wallet`
- public key: `~/.ssh/id_wallet.pub`

Important: in the current code, `wallet ssh-keygen --wallet ...` does not read the stored mnemonic back out of the keystore. If `--mnemonic` is omitted, it generates a fresh 24-word phrase and derives the SSH key from that.

## Data and Configuration

Default paths:

- config file: `~/.wallet/config.toml`
- keystore directory: `~/.wallet/keystore`
- wallet metadata index: `~/.wallet/keystore/metadata.json`

Global CLI options:

- `--config <PATH>`: override the config file path
- `--keystore <PATH>`: override the keystore directory
- `--json`: print machine-readable JSON
- `--verbose`: enable verbose CLI logging

The default config currently includes entries for:

- `ethereum`
- `polygon`
- `bitcoin`
- `solana`

## Workspace Layout

The active Cargo workspace currently includes:

- `wallet-cli`: the `wallet` binary and CLI command handlers
- `tcx-keystore`: encrypted keystore creation, serialization, and unlock flows
- `tcx-crypto`: shared cryptographic primitives
- `tcx-primitive`: mnemonic and key primitives used across the workspace
- `tcx-constants`: chain metadata and derivation helpers
- `tcx-evm`: EVM address and signing helpers
- `tcx-bitcoin-bdk`: Bitcoin address/signing helpers
- `tcx-solana`: Solana address/signing helpers
- `tcx-ssh`: mnemonic-based SSH key generation
- legacy chain crates from token-core such as `tcx-atom`, `tcx-eos`, `tcx-ton`, and `tcx-tron`

Some directories remain in the repository but are not active workspace members right now, including `tcx`, `tcx-substrate`, and `tcx-migration`.

For more detail, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## Development

Build the workspace:

```bash
cargo build
```

Build the CLI only:

```bash
cargo build -p wallet-cli
```

Run the CLI locally:

```bash
cargo run -p wallet-cli -- --help
```

Run focused tests that match the currently active CLI work:

```bash
cargo test -p wallet-cli
cargo test -p tcx-ssh
```

Run the full active workspace test suite:

```bash
cargo test --all
```

Format the code:

```bash
cargo fmt --all
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Security

See [SECURITY.md](./SECURITY.md).

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE).

## Acknowledgements

- The project is based on the work of the imToken team in [token-core-monorepo](https://github.com/consenlabs/token-core-monorepo).
- The SSH key generation flow is inspired by [mnemossh](https://github.com/abkvme/mnemossh).
