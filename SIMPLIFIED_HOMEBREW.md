# 简化的 Homebrew 安装方法

## 🎯 最简单的方案：直接从项目仓库安装

用户可以直接从您的项目仓库安装，无需创建额外的 tap 仓库：

```bash
# 直接从项目仓库安装
brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust
```

或者：

```bash
# 使用项目 URL 安装
brew install https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/Formula/mxlogger-analyzer-rust.rb
```

## 📋 实施步骤

### 1. 等待 GitHub Actions 完成
确保 GitHub Release 已创建并包含所有二进制文件。

### 2. 更新 Formula SHA256
```bash
./scripts/update_formula.sh 0.1.0
```

### 3. 提交更新的 Formula
```bash
git add Formula/mxlogger-analyzer-rust.rb
git commit -m "Update formula SHA256 for v0.1.0"
git push origin main
```

### 4. 测试安装
```bash
brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust
```

## ✅ 这样做的优势

1. **无需创建新仓库** - 一切都在当前项目中
2. **维护简单** - 只需要管理一个仓库
3. **自动更新** - formula 和代码在同一个仓库中
4. **立即可用** - 不需要等待审核或额外设置

## 📖 用户文档更新

在 README 中，用户安装说明变为：

```bash
# 安装方法
brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust
```

这比创建独立的 tap 仓库简单得多！
