# 超简单 Homebrew 发布

## 🎯 无需创建新仓库！

实际上，我们可以直接从项目仓库提供 Homebrew 安装，无需创建独立的 tap 仓库。

## 简单步骤

1. **等待 GitHub Actions 完成** (查看 https://github.com/suyulin/mxlogger_analyzer_rust/actions)

2. **更新 Formula SHA256**：
   ```bash
   ./scripts/update_formula.sh 0.1.0
   ```

3. **提交更新**：
   ```bash
   git add Formula/mxlogger-analyzer-rust.rb
   git commit -m "Update formula SHA256 for v0.1.0"
   git push origin main
   ```

4. **测试安装**：
   ```bash
   brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust
   ```

就这么简单！🎉

## 用户安装指南

用户现在可以通过以下方式安装您的工具：

```bash
# 方法 1：直接安装（推荐）
brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust

# 方法 2：使用 URL
brew install https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/Formula/mxlogger-analyzer-rust.rb
```

## 优势

- ✅ **无需创建新仓库**
- ✅ **维护简单** - 一切都在一个地方
- ✅ **立即可用** - 无需额外设置
- ✅ **自动同步** - formula 和代码在同一仓库
