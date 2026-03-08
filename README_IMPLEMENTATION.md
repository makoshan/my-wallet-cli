# my-wallet-cli: 完整实现文档

**项目名称**: my-wallet-cli  
**版本**: 0.1.0  
**作者**: Manus AI  
**日期**: 2026年3月8日

---

## 概述

`my-wallet-cli` 是一个基于 `token-core-monorepo` 的多链钱包命令行工具。它支持 EVM、Bitcoin、Solana 三大主流区块链，并提供了一个额外的功能：从助记词生成 SSH 密钥。

该项目采用了 Rust Workspace 模式，包含以下核心模块：

- **tcx-evm**: EVM 链支持（使用 Alloy）
- **tcx-bitcoin-bdk**: Bitcoin 支持（使用 BDK）
- **tcx-solana**: Solana 支持（使用 solana-sdk）
- **tcx-ssh**: SSH 密钥生成（基于 mnemossh）
- **wallet-cli**: 命令行应用层

---

## 项目结构

```
my-wallet-cli/
├── token-core/
│   ├── tcx/                    # 主入口（已有）
│   ├── tcx-eth/                # Ethereum 支持（已有）
│   ├── tcx-btc-kin/            # Bitcoin 支持（已有）
│   ├── tcx-keystore/           # Keystore 加密存储（已有）
│   ├── tcx-crypto/             # 密码学基础（已有）
│   │
│   ├── tcx-evm/                # 新增：通用 EVM 链支持
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── address.rs      # EVM 地址处理
│   │       ├── signer.rs       # 签名实现
│   │       └── transaction.rs  # 交易数据结构
│   │
│   ├── tcx-bitcoin-bdk/        # 新增：BDK Bitcoin 支持
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── address.rs
│   │       ├── signer.rs
│   │       └── transaction.rs
│   │
│   ├── tcx-solana/             # 新增：Solana 支持
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── address.rs
│   │       ├── signer.rs
│   │       └── transaction.rs
│   │
│   ├── tcx-ssh/                # 新增：SSH 密钥生成
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── mnemonic.rs     # 助记词处理
│   │       └── ssh_keys.rs     # SSH 密钥生成
│   │
│   └── wallet-cli/             # 新增：CLI 应用层
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs         # 程序入口
│           ├── config.rs       # 配置管理
│           ├── keystore_manager.rs
│           ├── output.rs       # 输出格式化
│           └── commands/
│               ├── mod.rs
│               ├── create.rs
│               ├── list.rs
│               ├── address.rs
│               ├── balance.rs
│               ├── sign_message.rs
│               ├── send.rs
│               ├── export.rs
│               ├── delete.rs
│               └── ssh_keygen.rs
│
└── README_IMPLEMENTATION.md    # 本文档
```

---

## 核心功能

### 1. 钱包管理

#### 创建钱包
```bash
wallet create --name my_wallet --password mypassword
```

#### 列出所有钱包
```bash
wallet list
```

#### 删除钱包
```bash
wallet delete --wallet my_wallet --force
```

#### 导出钱包
```bash
wallet export --wallet my_wallet --format json
```

### 2. 地址和余额

#### 显示钱包地址
```bash
# EVM 地址
wallet address --chain ethereum --wallet my_wallet

# Bitcoin 地址
wallet address --chain bitcoin --wallet my_wallet

# Solana 地址
wallet address --chain solana --wallet my_wallet
```

#### 查询余额
```bash
wallet balance --chain ethereum --wallet my_wallet
wallet balance --chain bitcoin --wallet my_wallet
wallet balance --chain solana --wallet my_wallet
```

### 3. 消息签名和交易

#### 签名消息
```bash
wallet sign-message "Hello, World!" --chain ethereum --wallet my_wallet
```

#### 发送交易
```bash
wallet send 1.0 --to 0x1234... --chain ethereum --wallet my_wallet
```

### 4. SSH 密钥生成（新功能）

#### 从助记词生成 SSH 密钥
```bash
# 从钱包的助记词生成
wallet ssh-keygen --wallet my_wallet --output ~/.ssh/id_ed25519_wallet

# 从指定的助记词生成
wallet ssh-keygen --mnemonic "word1 word2 ... word24" --output ~/.ssh/id_ed25519

# 带注释
wallet ssh-keygen --wallet my_wallet --comment "my-ssh-key" --output ~/.ssh/id_ed25519
```

---

## 配置文件

配置文件位置: `~/.wallet/config.toml`

```toml
[default]
wallet = "default"
chain = "ethereum"

[chains.ethereum]
rpc = "https://eth.llamarpc.com"
chain_id = 1

[chains.polygon]
rpc = "https://polygon-rpc.com"
chain_id = 137

[chains.bitcoin]
network = "mainnet"

[chains.solana]
rpc = "https://api.mainnet-beta.solana.com"

[keystore]
path = "~/.wallet/keystore"
encryption = "aes-256-gcm"
kdf = "argon2id"
```

---

## 编译和安装

### 编译

```bash
cd my-wallet-cli/token-core/wallet-cli
cargo build --release
```

### 生成的二进制文件

```bash
./target/release/wallet --version
./target/release/wallet --help
```

### 安装到系统路径

```bash
cargo install --path my-wallet-cli/token-core/wallet-cli
```

---

## 使用示例

### 创建钱包并生成地址

```bash
# 创建钱包
$ wallet create --name mywalletname --password mypassword
✓ Wallet created successfully

# 查看钱包列表
$ wallet list
Wallets:
  - mywalletname (created: 2026-03-08T10:00:00Z)

# 生成 Ethereum 地址
$ wallet address --chain ethereum --wallet mywalletname
Address: 0x1234567890123456789012345678901234567890
Chain: ethereum
Index: 0

# 生成 Bitcoin 地址
$ wallet address --chain bitcoin --wallet mywalletname
Address: 1A1z7agoat2LWQLZLV37ZLX4My6ps6nFX
Chain: bitcoin
Index: 0

# 生成 Solana 地址
$ wallet address --chain solana --wallet mywalletname
Address: 11111111111111111111111111111111
Chain: solana
Index: 0
```

### 签名和交易

```bash
# 签名消息
$ wallet sign-message "Hello, World!" --chain ethereum --wallet mywalletname
✓ Message signed successfully
Signature: 0x...

# 发送交易
$ wallet send 1.0 --to 0x... --chain ethereum --wallet mywalletname
✓ Transaction sent
TX Hash: 0x...
Status: pending
```

### SSH 密钥生成

```bash
# 从助记词生成 SSH 密钥
$ wallet ssh-keygen --wallet mywalletname --output ~/.ssh/id_ed25519_wallet
✓ SSH key generated successfully
Private Key: ~/.ssh/id_ed25519_wallet
Public Key: ~/.ssh/id_ed25519_wallet.pub
Fingerprint (MD5): MD5:xx:xx:xx:...
Fingerprint (SHA256): SHA256:...

# 查看生成的公钥
$ cat ~/.ssh/id_ed25519_wallet.pub
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... Generated SSH key
```

---

## 输出格式

### 文本格式（默认）

```bash
$ wallet address --chain ethereum
Address: 0x1234567890123456789012345678901234567890
Chain: ethereum
Index: 0
```

### JSON 格式

```bash
$ wallet address --chain ethereum --json
{
  "address": {
    "address": "0x1234567890123456789012345678901234567890",
    "chain": "ethereum",
    "wallet": "default",
    "index": 0
  }
}
```

---

## 架构设计

### 分层架构

```
┌─────────────────────────────────────┐
│      wallet-cli (CLI 应用层)        │
├─────────────────────────────────────┤
│  commands (地址、余额、签名、SSH)    │
├─────────────────────────────────────┤
│  tcx-evm | tcx-bitcoin-bdk | tcx-solana | tcx-ssh
├─────────────────────────────────────┤
│  tcx (主库) + tcx-keystore + tcx-crypto
├─────────────────────────────────────┤
│  底层密码学库 (secp256k1, ed25519等)  │
└─────────────────────────────────────┘
```

### 模块职责

| 模块 | 职责 |
| :--- | :--- |
| **wallet-cli** | 命令行界面、参数解析、输出格式化 |
| **tcx-evm** | EVM 链的地址、交易、签名 |
| **tcx-bitcoin-bdk** | Bitcoin 链的地址、PSBT、签名 |
| **tcx-solana** | Solana 链的地址、交易、签名 |
| **tcx-ssh** | 从助记词生成 SSH 密钥 |
| **tcx** | 核心 Keystore、密钥派生、RPC 接口 |

---

## SSH 密钥生成详解

### 工作原理

1. **助记词生成**: 使用 BIP-39 生成 12/18/24 字的助记词
2. **种子派生**: 使用 PBKDF2 从助记词派生 64 字节的种子
3. **密钥生成**: 使用种子的前 32 字节作为 Ed25519 私钥
4. **格式化**: 将密钥格式化为 OpenSSH 格式
5. **保存**: 保存到 `~/.ssh/` 目录，私钥权限为 0600

### 支持的密钥类型

- **ssh-ed25519**: 现代、安全、推荐使用

### 指纹格式

- **MD5**: `MD5:xx:xx:xx:...` (传统格式)
- **SHA256**: `SHA256:...` (现代格式)

### 安全特性

- 私钥在内存中使用 `ZeroizeOnDrop` 自动清零
- 私钥文件权限设置为 0600（仅所有者可读写）
- 支持密码保护（可选）

---

## 测试

### 单元测试

```bash
cd my-wallet-cli/token-core/tcx-evm
cargo test

cd my-wallet-cli/token-core/tcx-bitcoin-bdk
cargo test

cd my-wallet-cli/token-core/tcx-solana
cargo test

cd my-wallet-cli/token-core/tcx-ssh
cargo test
```

### 集成测试

```bash
cd my-wallet-cli/token-core/wallet-cli
cargo test
```

### 手动测试

```bash
# 编译
cargo build --release

# 创建钱包
./target/release/wallet create --name test_wallet

# 生成地址
./target/release/wallet address --chain ethereum --wallet test_wallet

# 生成 SSH 密钥
./target/release/wallet ssh-keygen --wallet test_wallet --output /tmp/test_key
```

---

## 依赖关系

### 核心依赖

| 库 | 版本 | 用途 |
| :--- | :--- | :--- |
| `alloy` | 0.7+ | EVM 支持 |
| `bdk` | 0.28+ | Bitcoin 支持 |
| `solana-sdk` | 1.18+ | Solana 支持 |
| `ed25519-dalek` | 2.1+ | Ed25519 签名 |
| `bip39` | 0.10+ | BIP-39 助记词 |
| `clap` | 4.4+ | CLI 参数解析 |
| `tokio` | 1.35+ | 异步运行时 |

---

## 安全考虑

### 密钥管理

- ✅ 私钥永远不离开本地存储
- ✅ Keystore 使用 AES-256-GCM 加密
- ✅ 密码使用 Argon2id 派生
- ✅ 内存中的敏感数据使用 ZeroizeOnDrop 清零

### 输入验证

- ✅ 所有地址输入进行格式验证
- ✅ 交易参数进行类型检查
- ✅ 助记词进行 BIP-39 验证

### 网络安全

- ✅ RPC 调用使用 HTTPS
- ✅ 支持自定义 RPC 端点
- ✅ 不记录敏感信息到日志

---

## 故障排除

### 常见问题

**Q: 编译时出现依赖版本冲突**

A: 清除 Cargo 缓存并重新编译：
```bash
cargo clean
cargo build --release
```

**Q: SSH 密钥生成失败**

A: 确保 `~/.ssh` 目录存在：
```bash
mkdir -p ~/.ssh
chmod 700 ~/.ssh
```

**Q: 钱包文件损坏**

A: 检查 `~/.wallet/keystore` 目录的权限和完整性。

---

## 未来改进

### 短期（1-2 个月）

- [ ] 支持硬件钱包集成（Ledger, Trezor）
- [ ] 支持多签钱包
- [ ] 支持更多 EVM 链
- [ ] 性能优化

### 中期（3-6 个月）

- [ ] 支持 MPC 钱包
- [ ] 支持账户抽象 (ERC-4337)
- [ ] 支持跨链桥接
- [ ] Web UI

### 长期（6+ 个月）

- [ ] 支持更多区块链
- [ ] 支持智能合约交互
- [ ] 支持 DeFi 集成
- [ ] 支持 AI 驱动的风险评估

---

## 许可证

MIT License

---

## 贡献

欢迎提交 Issue 和 Pull Request！

---

## 联系方式

如有问题或建议，请通过以下方式联系：

- GitHub Issues: https://github.com/your-repo/issues
- Email: support@example.com

---

## 更新日志

### v0.1.0 (2026-03-08)

- ✅ 初始版本发布
- ✅ 支持 EVM、Bitcoin、Solana
- ✅ 支持 SSH 密钥生成
- ✅ 完整的 CLI 工具
- ✅ 配置文件支持
- ✅ JSON 输出格式

---

**感谢使用 my-wallet-cli！**
