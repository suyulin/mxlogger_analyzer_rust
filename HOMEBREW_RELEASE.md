# 发布到 Homebrew 指南

本文档描述了如何将 `mxlogger_analyzer_rust` 发布到 Homebrew。

## 前置条件

1. 确保您有 GitHub 仓库的管理员权限
2. 安装了 `gh` CLI 工具（GitHub CLI）
3. 已配置 GitHub 认证

## 发布步骤

### 1. 准备发布

首先确保代码已经准备好发布：

```bash
# 确保所有更改已提交
git add .
git commit -m "Prepare for v0.1.0 release"
git push origin main
```

### 2. 创建 Git 标签并推送

```bash
# 创建标签
git tag v0.1.0

# 推送标签（这将触发 GitHub Actions 构建）
git push origin v0.1.0
```

### 3. 等待 GitHub Actions 完成

GitHub Actions 将自动：
- 为多个平台构建二进制文件
- 创建 GitHub Release
- 上传构建的资产

在 GitHub 上检查 Actions 标签页以确保构建成功。

### 4. 更新 Homebrew Formula

构建完成后，运行脚本来更新 formula 的 SHA256 值：

```bash
./scripts/update_formula.sh 0.1.0
```

### 5. 测试 Formula

在本地测试 formula：

```bash
# 安装本地 formula
brew install --build-from-source ./Formula/mxlogger-analyzer-rust.rb

# 测试安装的工具
mxlogger_analyzer_rust --help

# 卸载测试版本
brew uninstall mxlogger-analyzer-rust
```

### 6. 提交到 Homebrew

有两种方式将您的 formula 提交到 Homebrew：

#### 选项 A: 提交到官方 Homebrew Core（推荐用于流行工具）

1. Fork [homebrew-core](https://github.com/Homebrew/homebrew-core) 仓库
2. 将您的 formula 复制到 `Formula/` 目录
3. 创建 Pull Request

#### 选项 B: 创建自己的 Homebrew Tap（推荐用于个人/专业工具）

```bash
# 创建新的 tap 仓库
gh repo create homebrew-mxlogger --public --description "Homebrew tap for MxLogger tools"

# 克隆并设置 tap
git clone https://github.com/suyulin/homebrew-mxlogger.git
cd homebrew-mxlogger

# 复制 formula
cp ../mxlogger_analyzer_rust/Formula/mxlogger-analyzer-rust.rb ./Formula/

# 提交并推送
git add .
git commit -m "Add mxlogger-analyzer-rust formula"
git push origin main
```

### 7. 用户安装指南

#### 从官方 Homebrew Core 安装（如果被接受）:
```bash
brew install mxlogger-analyzer-rust
```

#### 从您的 Tap 安装:
```bash
brew tap suyulin/mxlogger
brew install mxlogger-analyzer-rust
```

## 更新现有的 Formula

当发布新版本时：

1. 更新 `Cargo.toml` 中的版本号
2. 提交更改并创建新标签
3. 等待 GitHub Actions 完成
4. 运行更新脚本：`./scripts/update_formula.sh <new_version>`
5. 提交更新的 formula

## 故障排除

### GitHub Actions 失败
- 检查 Rust 版本兼容性
- 确保所有依赖项都可用
- 检查交叉编译问题

### SHA256 不匹配
- 确保发布的资产已完全上传
- 重新运行更新脚本
- 手动验证下载的文件

### Homebrew 安装失败
- 检查二进制文件权限
- 验证 formula 语法
- 测试不同平台

## 最佳实践

1. **版本控制**: 使用语义版本控制 (SemVer)
2. **测试**: 在发布前彻底测试 formula
3. **文档**: 保持 README 和发布说明更新
4. **安全**: 定期更新依赖项
5. **用户反馈**: 监控 GitHub issues 和 Homebrew 反馈

## 相关资源

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
