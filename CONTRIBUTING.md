# 贡献指南

感谢您对 `my-wallet-cli` 项目的兴趣！我们欢迎所有形式的贡献。

## 🤝 如何贡献

### 报告 Bug

如果您发现了 bug，请创建一个 Issue：

1. 访问 [Issues](https://github.com/makoshan/my-wallet-cli/issues)
2. 点击 "New Issue"
3. 提供清晰的标题和描述
4. 包含重现步骤
5. 附加相关的日志或错误信息

### 提交功能请求

如果您有功能建议，请创建一个 Issue：

1. 访问 [Issues](https://github.com/makoshan/my-wallet-cli/issues)
2. 点击 "New Issue"
3. 清晰地描述您想要的功能
4. 解释为什么这个功能有用
5. 提供可能的实现方案

### 提交代码

#### 1. Fork 项目

```bash
# 访问 https://github.com/makoshan/my-wallet-cli
# 点击 "Fork" 按钮
```

#### 2. Clone 您的 Fork

```bash
git clone https://github.com/YOUR_USERNAME/my-wallet-cli.git
cd my-wallet-cli
```

#### 3. 创建特性分支

```bash
git checkout -b feature/your-feature-name
```

#### 4. 进行更改

- 编写清晰、有注释的代码
- 遵循 Rust 代码风格指南
- 添加必要的测试

#### 5. 测试您的更改

```bash
# 运行所有测试
cargo test --all

# 运行 clippy 检查
cargo clippy --all-targets --all-features -- -D warnings

# 检查代码格式
cargo fmt --all -- --check
```

#### 6. 提交更改

```bash
git add .
git commit -m "feat: add your feature description"
```

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码风格（不改变功能）
- `refactor:` 代码重构
- `perf:` 性能优化
- `test:` 添加或修改测试
- `chore:` 构建或依赖更新

#### 7. 推送到您的 Fork

```bash
git push origin feature/your-feature-name
```

#### 8. 创建 Pull Request

1. 访问您的 Fork
2. 点击 "Compare & pull request"
3. 填写 PR 描述
4. 点击 "Create pull request"

## 📋 代码质量标准

所有贡献必须满足以下要求：

### 代码风格

- 使用 `cargo fmt` 格式化代码
- 遵循 Rust 命名约定
- 添加必要的注释和文档

### 测试

- 为新功能添加单元测试
- 为 bug 修复添加回归测试
- 确保所有测试通过

### 文档

- 更新相关文档
- 添加 API 文档注释
- 更新 README（如需要）

### 安全

- 运行 `cargo audit` 检查依赖安全
- 不提交敏感信息（密钥、密码等）
- 遵循安全最佳实践

## 🔍 审查过程

1. **自动检查**: GitHub Actions 会运行 CI/CD 流程
2. **代码审查**: 维护者会审查您的代码
3. **反馈**: 如需要，我们会提供反馈
4. **合并**: 审查通过后，您的 PR 会被合并

## 📚 开发指南

### 项目结构

```
my-wallet-cli/
├── token-core/
│   ├── tcx-evm/           # EVM 链支持
│   ├── tcx-bitcoin-bdk/   # Bitcoin 支持
│   ├── tcx-solana/        # Solana 支持
│   ├── tcx-ssh/           # SSH 密钥生成
│   └── wallet-cli/        # CLI 应用
├── docs/                  # 文档
└── ...
```

### 添加新链支持

1. 在 `token-core/` 下创建新的 crate（如 `tcx-newchain`）
2. 实现必要的 trait（Address, Signer, Transaction）
3. 在 `wallet-cli` 中添加支持
4. 添加测试和文档

### 添加新命令

1. 在 `wallet-cli/src/commands/` 下创建新文件
2. 在 `commands/mod.rs` 中导入
3. 在 `main.rs` 中添加命令定义
4. 实现命令逻辑
5. 添加测试

## 🐛 调试

### 启用详细日志

```bash
RUST_LOG=debug cargo run -- address --chain ethereum
```

### 使用调试器

```bash
# 使用 gdb
rust-gdb ./target/debug/wallet

# 使用 lldb (macOS)
lldb ./target/debug/wallet
```

## 📖 资源

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)

## 💬 讨论

- [GitHub Discussions](https://github.com/makoshan/my-wallet-cli/discussions)
- [GitHub Issues](https://github.com/makoshan/my-wallet-cli/issues)

## 📝 许可证

通过提交代码，您同意您的代码将在 MIT 许可证下发布。

---

感谢您的贡献！🎉
