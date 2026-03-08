# Architecture

This document provides a detailed overview of the `my-wallet-cli` architecture. The design prioritizes modularity, security, and extensibility.

## 1. Core Principles

- **Layered Architecture**: Code is organized into distinct layers, from the user-facing CLI down to the cryptographic primitives.
- **Trait-Based Interfaces**: Chains and cryptographic functions are abstracted behind Rust traits, allowing for polymorphism and easy extension.
- **Workspace Structure**: The project is a single Cargo workspace, enabling unified dependency management, building, and testing.
- **Security First**: Sensitive operations and data handling are isolated in dedicated, audited crates.

## 2. Project Structure (Flattened Workspace)

The project was refactored from the original `token-core-monorepo` to have a flat structure. All crates are now in the root directory, simplifying navigation and build configurations.

```
my-wallet-cli/
├── Cargo.toml         # Workspace definition
├── wallet-cli/        # CLI Application Layer
├── tcx/               # Core API Facade Layer
├── tcx-keystore/      # Keystore & Encryption Layer
├── tcx-crypto/        # Cryptographic Primitives Layer
├── tcx-primitive/     # Shared Data Structures
├── tcx-evm/           # EVM Chain Adapter
├── tcx-bitcoin-bdk/   # Bitcoin Chain Adapter
├── tcx-solana/        # Solana Chain Adapter
├── tcx-ssh/           # SSH Key Generation Adapter
└── ... (other tcx-* chain adapters)
```

## 3. Layer Breakdown

### Layer 1: CLI Application (`wallet-cli`)

- **Responsibility**: Handles all user interaction, command parsing, and output formatting.
- **Framework**: Uses `clap` for command-line argument parsing.
- **Functionality**: 
    - Defines the command structure (`wallet <COMMAND> [OPTIONS]`).
    - Manages configuration files (`~/.wallet/config.toml`).
    - Interacts with the Keystore Manager to handle wallet files.
    - Dispatches commands to the appropriate chain adapters.
    - Formats and prints results (JSON or human-readable text).
- **Security**: This layer **never** handles raw private keys. It only deals with wallet identifiers and encrypted keystore data.

### Layer 2: Core API Facade (`tcx`)

- **Responsibility**: Provides a single, unified entry point for high-level wallet operations. This was originally designed to serve a protobuf-based API for mobile clients.
- **Functionality**:
    - Exports functions like `tcx_api_address`, `tcx_api_sign_transaction`.
    - Maps chain identifiers (e.g., "ethereum") to the correct chain adapter.
    - Handles serialization and deserialization of API requests and responses.
- **Note**: While the CLI doesn't strictly need this layer (it could call adapters directly), we retain it to maintain compatibility with the original token-core architecture and to provide a clear API boundary.

### Layer 3: Chain Adapters (`tcx-*`)

- **Responsibility**: Implements the logic for a specific blockchain or a family of blockchains.
- **Key Traits**: Each adapter typically implements a set of core traits defined in `tcx-primitive`:
    - `AddressManage`: For deriving addresses from a public key.
    - `TransactionSigner`: For signing transactions.
    - `MessageSigner`: For signing arbitrary messages.
- **Examples**:
    - `tcx-evm`: Uses the **Alloy** library. Implements `AddressManage` for EIP-55 addresses and `TransactionSigner` for EIP-1559 transactions.
    - `tcx-bitcoin-bdk`: Uses the **BDK** library. Implements `AddressManage` for SegWit/Taproot addresses and `TransactionSigner` for PSBTs.
    - `tcx-solana`: Uses the **Solana SDK**. Implements `AddressManage` for Base58 addresses and `TransactionSigner` for Solana transactions.

### Layer 4: Keystore & Encryption (`tcx-keystore`)

- **Responsibility**: Securely stores and retrieves encrypted private keys.
- **Functionality**:
    - Defines the `Keystore` struct, which is serialized to a JSON file.
    - Uses **Argon2id** for password-based key derivation.
    - Uses **AES-256-GCM** to encrypt the private key.
    - Implements BIP39 mnemonic generation and derivation.
    - Implements BIP32/BIP44 hierarchical derivation paths.
- **Security**: This is a critical security component. It ensures that private keys are never stored in plaintext.

### Layer 5: Cryptographic Primitives (`tcx-crypto`, `tcx-primitive`)

- **Responsibility**: Provides the fundamental building blocks for all cryptographic operations.
- **`tcx-primitive`**: Defines shared data structures and traits.
    - `PrivateKey`, `PublicKey`, `Signature` structs.
    - `AddressManage`, `TransactionSigner` traits.
    - `Bip39` mnemonic handling.
- **`tcx-crypto`**: Provides hashing functions and other cryptographic utilities.
    - Wrappers for SHA-256, Keccak-256, Blake2b, etc.

## 4. Data Flow Example: `wallet address --chain ethereum`

1.  **`wallet-cli`**: `clap` parses the command.
2.  **`wallet-cli`**: The `address` command logic is executed.
3.  **`wallet-cli`**: It loads the encrypted keystore file for the specified wallet.
4.  **`wallet-cli`**: It prompts the user for their password.
5.  **`tcx-keystore`**: The CLI calls `keystore.unlock(password)`.
    - Argon2id verifies the password.
    - AES-256-GCM decrypts the master private key into a secure memory region.
6.  **`tcx-keystore`**: The CLI calls `keystore.derive_private_key(path)` with the Ethereum derivation path (`m/44'/60'/0'/0/0`).
7.  **`tcx-primitive`**: The derived private key is used to generate the corresponding public key.
8.  **`tcx-evm`**: The public key is passed to the `AddressManage::get_address` implementation in the EVM adapter.
9.  **`tcx-evm`**: It computes the Keccak-256 hash of the public key, takes the last 20 bytes, and applies EIP-55 checksum encoding.
10. **`wallet-cli`**: The resulting address string is returned to the CLI.
11. **`wallet-cli`**: The address is formatted and printed to the console.
12. **`tcx-keystore`**: The decrypted private key (and any derived keys) are securely zeroed from memory when they go out of scope (thanks to `ZeroizeOnDrop`).

## 5. Extensibility

Adding a new chain (e.g., "NewChain") is straightforward:

1.  **Create Crate**: Create a new crate `tcx-newchain`.
2.  **Add Dependencies**: Add the necessary SDK for NewChain (e.g., `newchain-sdk`) to its `Cargo.toml`.
3.  **Implement Traits**: In `tcx-newchain`, implement the `AddressManage` and `TransactionSigner` traits for NewChain's specific logic.
4.  **Update Workspace**: Add `"tcx-newchain"` to the `members` list in the root `Cargo.toml`.
5.  **Update `tcx` Facade**: In the `tcx` crate, add a match arm to dispatch calls for the "newchain" identifier to your new crate.
6.  **Update `wallet-cli`**: Add "newchain" as a valid option for the `--chain` argument.
7.  **Add Tests**: Create unit and integration tests for the new chain.
