# 快速发布到 Homebrew

## 简单步骤

1. **运行发布脚本**：
   ```bash
   ./scripts/release.sh 0.1.0
   ```

2. **等待 GitHub Actions 完成** (大约 5-10 分钟)

3. **创建您的 Homebrew Tap**：
   ```bash
   gh repo create homebrew-mxlogger --public --description "Homebrew tap for MxLogger tools"
   ```

4. **设置 Tap 仓库**：
   ```bash
   git clone https://github.com/suyulin/homebrew-mxlogger.git
   cd homebrew-mxlogger
   mkdir Formula
   cp ../mxlogger_analyzer_rust/Formula/mxlogger-analyzer-rust.rb ./Formula/
   git add .
   git commit -m "Add mxlogger-analyzer-rust formula"
   git push origin main
   ```

5. **测试安装**：
   ```bash
   brew tap suyulin/mxlogger
   brew install mxlogger-analyzer-rust
   ```

## 用户安装指南

用户现在可以通过以下方式安装您的工具：

```bash
brew tap suyulin/mxlogger
brew install mxlogger-analyzer-rust
```

或者一行命令：
```bash
brew install suyulin/mxlogger/mxlogger-analyzer-rust
```

## 更新发布

当您需要发布新版本时：

1. 运行发布脚本：`./scripts/release.sh <new_version>`
2. 等待 GitHub Actions 完成
3. 更新 Homebrew Tap：
   ```bash
   cd ../homebrew-mxlogger
   cp ../mxlogger_analyzer_rust/Formula/mxlogger-analyzer-rust.rb ./Formula/
   git add .
   git commit -m "Update mxlogger-analyzer-rust to v<new_version>"
   git push origin main
   ```

就这么简单！🎉
