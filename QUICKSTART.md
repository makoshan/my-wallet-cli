# my-wallet-cli 快速开始指南

## 安装

### 前置要求

- Rust 1.70+ (通过 [rustup](https://rustup.rs/) 安装)
- Git
- Linux/macOS/Windows

### 编译和安装

```bash
# 克隆项目
git clone https://github.com/your-repo/my-wallet-cli.git
cd my-wallet-cli

# 编译
cd token-core/wallet-cli
cargo build --release

# 安装到系统路径
cargo install --path .

# 或者直接运行
./target/release/wallet --version
```

## 基本使用

### 1. 创建钱包

```bash
# 创建一个新钱包
wallet create --name my_wallet --password mypassword

# 输出:
# ✓ Wallet created successfully
# wallet: {
#   "wallet_id": "...",
#   "wallet_name": "my_wallet",
#   "status": "created"
# }
```

### 2. 列出所有钱包

```bash
wallet list

# 输出:
# wallets: [
#   {
#     "id": "...",
#     "name": "my_wallet",
#     "created_at": "2026-03-08T10:00:00Z"
#   }
# ]
```

### 3. 生成地址

```bash
# 生成 Ethereum 地址
wallet address --chain ethereum --wallet my_wallet

# 输出:
# address: {
#   "address": "0x1234567890123456789012345678901234567890",
#   "chain": "ethereum",
#   "wallet": "my_wallet",
#   "index": 0
# }

# 生成 Bitcoin 地址
wallet address --chain bitcoin --wallet my_wallet

# 生成 Solana 地址
wallet address --chain solana --wallet my_wallet
```

### 4. 查询余额

```bash
wallet balance --chain ethereum --wallet my_wallet

# 输出:
# balance: {
#   "balance": "1.5",
#   "chain": "ethereum",
#   "wallet": "my_wallet",
#   "unit": "ETH"
# }
```

### 5. 签名消息

```bash
wallet sign-message "Hello, World!" --chain ethereum --wallet my_wallet

# 输出:
# signature: {
#   "signature": "0x...",
#   "message": "Hello, World!",
#   "chain": "ethereum",
#   "wallet": "my_wallet",
#   "index": 0
# }
```

### 6. 发送交易

```bash
wallet send 1.0 --to 0x1234567890123456789012345678901234567890 \
  --chain ethereum --wallet my_wallet

# 输出:
# transaction: {
#   "tx_hash": "0x...",
#   "amount": "1.0",
#   "to": "0x...",
#   "chain": "ethereum",
#   "wallet": "my_wallet",
#   "status": "pending"
# }
```

### 7. 生成 SSH 密钥（新功能）

```bash
# 从钱包生成 SSH 密钥
wallet ssh-keygen --wallet my_wallet --output ~/.ssh/id_ed25519_wallet

# 输出:
# ssh_key: {
#   "status": "success",
#   "private_key_path": "~/.ssh/id_ed25519_wallet",
#   "public_key_path": "~/.ssh/id_ed25519_wallet.pub",
#   "public_key": "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI...",
#   "fingerprint_md5": "MD5:xx:xx:xx:...",
#   "fingerprint_sha256": "SHA256:...",
#   "key_type": "ssh-ed25519"
# }

# 查看生成的公钥
cat ~/.ssh/id_ed25519_wallet.pub

# 添加到 GitHub/GitLab
# 复制公钥内容到 https://github.com/settings/keys
```

### 8. 导出钱包

```bash
wallet export --wallet my_wallet --format json

# 输出:
# export: {
#   "wallet": "my_wallet",
#   "format": "json",
#   "status": "exported",
#   "file": "my_wallet.json"
# }
```

### 9. 删除钱包

```bash
wallet delete --wallet my_wallet

# 输出:
# ✓ Wallet 'my_wallet' deleted

# 或者跳过确认
wallet delete --wallet my_wallet --force
```

## JSON 输出格式

所有命令都支持 `--json` 标志来输出 JSON 格式的结果：

```bash
wallet address --chain ethereum --json

# 输出:
# {
#   "address": {
#     "address": "0x1234567890123456789012345678901234567890",
#     "chain": "ethereum",
#     "wallet": "default",
#     "index": 0
#   }
# }
```

## 配置文件

配置文件位置: `~/.wallet/config.toml`

### 查看默认配置

```bash
cat ~/.wallet/config.toml
```

### 自定义 RPC 端点

```toml
[chains.ethereum]
rpc = "https://your-custom-rpc.com"
chain_id = 1

[chains.polygon]
rpc = "https://polygon-rpc.com"
chain_id = 137

[chains.bitcoin]
network = "mainnet"

[chains.solana]
rpc = "https://api.mainnet-beta.solana.com"
```

## 常见任务

### 为 GitHub 生成 SSH 密钥

```bash
# 1. 创建钱包
wallet create --name github_key

# 2. 生成 SSH 密钥
wallet ssh-keygen --wallet github_key --comment "GitHub SSH Key" \
  --output ~/.ssh/github_ed25519

# 3. 添加到 SSH 代理
ssh-add ~/.ssh/github_ed25519

# 4. 复制公钥到 GitHub
cat ~/.ssh/github_ed25519.pub | pbcopy  # macOS
# 或
cat ~/.ssh/github_ed25519.pub | xclip -selection clipboard  # Linux

# 5. 在 GitHub 中添加密钥
# https://github.com/settings/keys -> New SSH key
```

### 多钱包管理

```bash
# 创建多个钱包
wallet create --name wallet1
wallet create --name wallet2
wallet create --name wallet3

# 列出所有钱包
wallet list

# 为不同钱包生成地址
wallet address --chain ethereum --wallet wallet1
wallet address --chain ethereum --wallet wallet2
wallet address --chain ethereum --wallet wallet3

# 为不同钱包生成 SSH 密钥
wallet ssh-keygen --wallet wallet1 --output ~/.ssh/wallet1_key
wallet ssh-keygen --wallet wallet2 --output ~/.ssh/wallet2_key
wallet ssh-keygen --wallet wallet3 --output ~/.ssh/wallet3_key
```

### 跨链操作

```bash
# 为同一钱包生成多条链的地址
wallet address --chain ethereum --wallet my_wallet
wallet address --chain bitcoin --wallet my_wallet
wallet address --chain solana --wallet my_wallet

# 为多条链签名消息
wallet sign-message "Test" --chain ethereum --wallet my_wallet
wallet sign-message "Test" --chain bitcoin --wallet my_wallet
wallet sign-message "Test" --chain solana --wallet my_wallet
```

## 故障排除

### 问题：命令找不到

**解决方案**：确保已安装到系统路径
```bash
cargo install --path my-wallet-cli/token-core/wallet-cli
```

### 问题：钱包文件损坏

**解决方案**：检查 `~/.wallet/keystore` 目录
```bash
ls -la ~/.wallet/keystore
```

### 问题：SSH 密钥权限错误

**解决方案**：确保 `~/.ssh` 目录权限正确
```bash
chmod 700 ~/.ssh
chmod 600 ~/.ssh/id_ed25519_wallet
```

### 问题：RPC 连接失败

**解决方案**：检查网络连接和 RPC 端点
```bash
# 测试 RPC 端点
curl https://eth.llamarpc.com

# 修改配置文件中的 RPC 端点
nano ~/.wallet/config.toml
```

## 高级用法

### 使用自定义配置文件

```bash
wallet address --config /path/to/custom/config.toml --chain ethereum
```

### 使用自定义 Keystore 目录

```bash
wallet address --keystore /path/to/custom/keystore --chain ethereum
```

### 详细输出

```bash
wallet address --verbose --chain ethereum
```

### 组合选项

```bash
wallet address --config ~/.wallet/config.toml \
  --keystore ~/.wallet/keystore \
  --chain ethereum \
  --json \
  --verbose
```

## 安全建议

1. **保护私钥**: 不要在命令行中输入私钥，使用 Keystore 存储
2. **备份钱包**: 定期备份 `~/.wallet/keystore` 目录
3. **强密码**: 使用强密码保护钱包
4. **SSH 密钥权限**: 确保 SSH 私钥权限为 0600
5. **不要共享**: 不要共享私钥或 Keystore 文件

## 更多帮助

```bash
# 查看全局帮助
wallet --help

# 查看命令帮助
wallet address --help
wallet balance --help
wallet sign-message --help
wallet send --help
wallet ssh-keygen --help
wallet create --help
wallet list --help
wallet export --help
wallet delete --help
```

## 下一步

- 阅读完整文档: [README_IMPLEMENTATION.md](README_IMPLEMENTATION.md)
- 查看源代码: [token-core/wallet-cli](token-core/wallet-cli)
- 提交问题: [GitHub Issues](https://github.com/your-repo/issues)
- 贡献代码: [GitHub Pull Requests](https://github.com/your-repo/pulls)

---

**祝您使用愉快！** 🚀
