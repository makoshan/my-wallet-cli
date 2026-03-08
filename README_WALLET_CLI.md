# my-wallet-cli

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/your-repo/my-wallet-cli.svg)](https://github.com/your-repo/my-wallet-cli/releases)

**多链加密货币钱包命令行工具** - 支持 EVM、Bitcoin、Solana，并可从助记词生成 SSH 密钥。

## 🌟 特性

- ✅ **多链支持**: EVM、Bitcoin、Solana
- ✅ **完整 CLI**: 8 个核心命令 + SSH 密钥生成
- ✅ **安全设计**: 密钥加密存储、内存安全
- ✅ **灵活配置**: TOML 配置文件支持
- ✅ **多种输出**: 文本和 JSON 格式
- ✅ **SSH 密钥**: 从助记词生成 Ed25519 SSH 密钥
- ✅ **跨平台**: Linux、macOS、Windows 支持

## 📦 快速开始

### 安装

```bash
# 从源码编译
git clone https://github.com/your-repo/my-wallet-cli.git
cd my-wallet-cli/token-core/wallet-cli
cargo install --path .
```

### 基本使用

```bash
# 创建钱包
wallet create --name my_wallet --password mypassword

# 生成地址
wallet address --chain ethereum --wallet my_wallet

# 生成 SSH 密钥
wallet ssh-keygen --wallet my_wallet --output ~/.ssh/id_ed25519
```

## 📚 文档

- **[快速开始](QUICKSTART.md)** - 5 分钟快速上手
- **[完整实现文档](README_IMPLEMENTATION.md)** - 详细的架构和功能说明
- **[部署指南](DEPLOYMENT.md)** - 编译、打包、发布、部署

## 🏗️ 项目结构

```
my-wallet-cli/
├── token-core/
│   ├── tcx-evm/                # EVM 链支持
│   ├── tcx-bitcoin-bdk/        # Bitcoin 支持
│   ├── tcx-solana/             # Solana 支持
│   ├── tcx-ssh/                # SSH 密钥生成
│   └── wallet-cli/             # CLI 应用
├── QUICKSTART.md               # 快速开始
├── README_IMPLEMENTATION.md    # 完整文档
└── DEPLOYMENT.md               # 部署指南
```

## 🔧 核心命令

| 命令 | 功能 |
| :--- | :--- |
| `wallet create` | 创建钱包 |
| `wallet list` | 列出钱包 |
| `wallet address` | 显示地址 |
| `wallet balance` | 查询余额 |
| `wallet sign-message` | 签名消息 |
| `wallet send` | 发送交易 |
| `wallet ssh-keygen` | 生成 SSH 密钥 |

## 🌐 支持的区块链

- **EVM**: Ethereum、Polygon、Arbitrum、Optimism、Base、Avalanche、BNB Chain
- **Bitcoin**: Mainnet、Testnet、Signet
- **Solana**: Mainnet、Devnet、Testnet

## 🔐 安全特性

- AES-256-GCM 密钥加密
- Argon2id 密码派生
- ZeroizeOnDrop 内存清零
- Ed25519 SSH 密钥

## 🚀 编译

```bash
cd token-core/wallet-cli
cargo build --release
```

## 🧪 测试

```bash
cargo test --all
```

## 📄 许可证

MIT License

## 🙏 致谢

- [token-core-monorepo](https://github.com/consenlabs/token-core-monorepo)
- [Alloy](https://github.com/alloy-rs/alloy)
- [BDK](https://bitcoindevkit.org/)
- [Solana SDK](https://github.com/solana-labs/solana)
- [mnemossh](https://github.com/abkvme/mnemossh)

---

**开始使用**: [快速开始指南](QUICKSTART.md) | **深入了解**: [完整文档](README_IMPLEMENTATION.md) | **部署**: [部署指南](DEPLOYMENT.md)
