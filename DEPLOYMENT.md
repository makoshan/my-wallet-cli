# my-wallet-cli 部署和发布指南

## 目录

1. [编译](#编译)
2. [打包](#打包)
3. [发布](#发布)
4. [Docker 部署](#docker-部署)
5. [CI/CD 配置](#cicd-配置)
6. [性能优化](#性能优化)
7. [安全检查](#安全检查)

---

## 编译

### 开发编译

```bash
cd token-core/wallet-cli
cargo build
```

### 发布编译

```bash
cd token-core/wallet-cli
cargo build --release
```

### 优化编译

编辑 `Cargo.toml` 添加优化配置：

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

然后编译：

```bash
cargo build --release
```

### 交叉编译

#### Linux x86_64

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

#### macOS x86_64

```bash
cargo build --release --target x86_64-apple-darwin
```

#### macOS ARM64 (Apple Silicon)

```bash
cargo build --release --target aarch64-apple-darwin
```

#### Windows x86_64

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

#### Linux ARM64

```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

---

## 打包

### 创建发布包

```bash
# 创建发布目录
mkdir -p releases

# Linux
cd token-core/wallet-cli
cargo build --release --target x86_64-unknown-linux-gnu
tar -czf ../../releases/wallet-linux-x86_64.tar.gz \
  target/x86_64-unknown-linux-gnu/release/wallet

# macOS
cargo build --release --target x86_64-apple-darwin
tar -czf ../../releases/wallet-macos-x86_64.tar.gz \
  target/x86_64-apple-darwin/release/wallet

# macOS ARM64
cargo build --release --target aarch64-apple-darwin
tar -czf ../../releases/wallet-macos-arm64.tar.gz \
  target/aarch64-apple-darwin/release/wallet

# Windows
cargo build --release --target x86_64-pc-windows-msvc
7z a ../../releases/wallet-windows-x86_64.zip \
  target/x86_64-pc-windows-msvc/release/wallet.exe
```

### 创建校验和

```bash
cd releases
sha256sum * > SHA256SUMS
gpg --detach-sign SHA256SUMS
```

---

## 发布

### 发布到 Cargo

#### 1. 准备 Cargo.toml

```toml
[package]
name = "wallet-cli"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
description = "Multi-chain cryptocurrency wallet CLI"
repository = "https://github.com/your-repo/my-wallet-cli"
homepage = "https://github.com/your-repo/my-wallet-cli"
documentation = "https://docs.rs/wallet-cli"
readme = "README.md"
license = "MIT"
keywords = ["wallet", "cryptocurrency", "blockchain", "cli"]
categories = ["command-line-utilities", "cryptography"]
```

#### 2. 登录 Cargo

```bash
cargo login
```

#### 3. 发布

```bash
cd token-core/wallet-cli
cargo publish
```

### 发布到 GitHub Releases

#### 1. 创建 GitHub Release

```bash
# 使用 GitHub CLI
gh release create v0.1.0 \
  releases/wallet-linux-x86_64.tar.gz \
  releases/wallet-macos-x86_64.tar.gz \
  releases/wallet-macos-arm64.tar.gz \
  releases/wallet-windows-x86_64.zip \
  releases/SHA256SUMS \
  releases/SHA256SUMS.asc \
  --title "Release v0.1.0" \
  --notes "Initial release"
```

#### 2. 或使用 Web UI

访问 https://github.com/your-repo/my-wallet-cli/releases/new

### 发布到包管理器

#### Homebrew (macOS)

创建 `Formula/wallet-cli.rb`：

```ruby
class WalletCli < Formula
  desc "Multi-chain cryptocurrency wallet CLI"
  homepage "https://github.com/your-repo/my-wallet-cli"
  url "https://github.com/your-repo/my-wallet-cli/releases/download/v0.1.0/wallet-macos-x86_64.tar.gz"
  sha256 "YOUR_SHA256_HASH"
  
  def install
    bin.install "wallet"
  end
  
  test do
    system "#{bin}/wallet", "--version"
  end
end
```

#### Scoop (Windows)

创建 `bucket/wallet-cli.json`：

```json
{
  "version": "0.1.0",
  "description": "Multi-chain cryptocurrency wallet CLI",
  "homepage": "https://github.com/your-repo/my-wallet-cli",
  "license": "MIT",
  "url": "https://github.com/your-repo/my-wallet-cli/releases/download/v0.1.0/wallet-windows-x86_64.zip",
  "hash": "YOUR_SHA256_HASH",
  "bin": "wallet.exe"
}
```

---

## Docker 部署

### 创建 Dockerfile

```dockerfile
# 构建阶段
FROM rust:1.70 as builder

WORKDIR /build
COPY . .

RUN cd token-core/wallet-cli && \
    cargo build --release

# 运行阶段
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/token-core/wallet-cli/target/release/wallet /usr/local/bin/

ENTRYPOINT ["wallet"]
```

### 构建 Docker 镜像

```bash
docker build -t wallet-cli:0.1.0 .
docker tag wallet-cli:0.1.0 wallet-cli:latest
```

### 运行 Docker 容器

```bash
# 创建钱包
docker run -v ~/.wallet:/root/.wallet wallet-cli create --name my_wallet

# 生成地址
docker run -v ~/.wallet:/root/.wallet wallet-cli address --chain ethereum

# 生成 SSH 密钥
docker run -v ~/.wallet:/root/.wallet -v ~/.ssh:/root/.ssh \
  wallet-cli ssh-keygen --wallet my_wallet --output /root/.ssh/id_ed25519
```

### 推送到 Docker Hub

```bash
docker login
docker tag wallet-cli:0.1.0 your-username/wallet-cli:0.1.0
docker push your-username/wallet-cli:0.1.0
docker push your-username/wallet-cli:latest
```

---

## CI/CD 配置

### GitHub Actions

创建 `.github/workflows/release.yml`：

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: wallet-linux-x86_64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: wallet-macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: wallet-macos-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: wallet-windows-x86_64.exe

    steps:
      - uses: actions/checkout@v3
      
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build
        run: |
          cd token-core/wallet-cli
          cargo build --release --target ${{ matrix.target }}
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.artifact }}
          path: token-core/wallet-cli/target/${{ matrix.target }}/release/wallet*

  release:
    name: Release
    needs: build
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Download artifacts
        uses: actions/download-artifact@v3
      
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            wallet-linux-x86_64/wallet
            wallet-macos-x86_64/wallet
            wallet-macos-arm64/wallet
            wallet-windows-x86_64.exe/wallet.exe
```

### GitLab CI

创建 `.gitlab-ci.yml`：

```yaml
stages:
  - build
  - test
  - release

build:linux:
  stage: build
  image: rust:1.70
  script:
    - cd token-core/wallet-cli
    - cargo build --release
  artifacts:
    paths:
      - token-core/wallet-cli/target/release/wallet

test:
  stage: test
  image: rust:1.70
  script:
    - cd token-core/wallet-cli
    - cargo test --release

release:
  stage: release
  image: rust:1.70
  script:
    - cd token-core/wallet-cli
    - cargo publish
  only:
    - tags
```

---

## 性能优化

### 编译时优化

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

### 运行时优化

1. **使用发布版本**
   ```bash
   cargo build --release
   ```

2. **启用 LTO (Link Time Optimization)**
   ```toml
   [profile.release]
   lto = true
   ```

3. **减少 codegen-units**
   ```toml
   [profile.release]
   codegen-units = 1
   ```

4. **移除调试符号**
   ```toml
   [profile.release]
   strip = true
   ```

### 基准测试

```bash
cd token-core/wallet-cli
cargo bench
```

---

## 安全检查

### 依赖安全审计

```bash
# 安装 cargo-audit
cargo install cargo-audit

# 运行审计
cargo audit
```

### 代码质量检查

```bash
# 安装 clippy
rustup component add clippy

# 运行 clippy
cargo clippy --all-targets --all-features

# 严格模式
cargo clippy --all-targets --all-features -- -D warnings
```

### 格式检查

```bash
# 安装 rustfmt
rustup component add rustfmt

# 检查格式
cargo fmt --check

# 自动格式化
cargo fmt
```

### 测试覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html --output-dir coverage
```

### 安全漏洞扫描

```bash
# 安装 cargo-deny
cargo install cargo-deny

# 初始化配置
cargo deny init

# 运行扫描
cargo deny check
```

---

## 版本管理

### 语义化版本 (Semantic Versioning)

遵循 MAJOR.MINOR.PATCH 格式：

- **MAJOR**: 不兼容的 API 变化
- **MINOR**: 向后兼容的功能增加
- **PATCH**: 向后兼容的 bug 修复

### 版本号更新

```bash
# 更新 Cargo.toml 中的版本
sed -i 's/version = "0.1.0"/version = "0.2.0"/' token-core/wallet-cli/Cargo.toml

# 创建 git 标签
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

---

## 监控和日志

### 启用日志

```bash
RUST_LOG=debug wallet address --chain ethereum
RUST_LOG=wallet_cli=debug wallet address --chain ethereum
```

### 日志级别

- **ERROR**: 错误信息
- **WARN**: 警告信息
- **INFO**: 一般信息
- **DEBUG**: 调试信息
- **TRACE**: 详细跟踪

---

## 故障排除

### 编译失败

```bash
# 清除缓存
cargo clean

# 更新依赖
cargo update

# 重新编译
cargo build --release
```

### 发布失败

```bash
# 检查 Cargo.toml 配置
cargo publish --dry-run

# 查看详细错误
cargo publish --verbose
```

### 性能问题

```bash
# 分析编译时间
cargo build -Z timings

# 分析运行时性能
cargo build --release
perf record ./target/release/wallet address --chain ethereum
perf report
```

---

## 检查清单

发布前的检查清单：

- [ ] 所有测试通过 (`cargo test`)
- [ ] 代码通过 clippy 检查 (`cargo clippy`)
- [ ] 代码格式正确 (`cargo fmt`)
- [ ] 没有安全漏洞 (`cargo audit`)
- [ ] 版本号已更新
- [ ] CHANGELOG 已更新
- [ ] README 已更新
- [ ] 所有文档已更新
- [ ] 创建了 git 标签
- [ ] 发布到 Cargo
- [ ] 发布到 GitHub Releases
- [ ] 发布到包管理器（可选）
- [ ] 发布 Docker 镜像（可选）

---

## 参考资源

- [Rust Book - Publishing](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Docker Documentation](https://docs.docker.com/)

---

**祝您部署顺利！** 🚀
