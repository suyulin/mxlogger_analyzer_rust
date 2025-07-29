#!/bin/bash
# 自动设置 Homebrew Tap 仓库

set -e

echo "🍺 设置 Homebrew Tap 仓库..."

# 检查是否在正确的目录
if [ ! -f "Formula/mxlogger-analyzer-rust.rb" ]; then
    echo "❌ 错误: 请在 mxlogger_analyzer_rust 项目根目录运行此脚本"
    exit 1
fi

# 克隆 tap 仓库
if [ ! -d "homebrew-mxlogger" ]; then
    echo "📥 克隆 homebrew-mxlogger 仓库..."
    git clone https://github.com/suyulin/homebrew-mxlogger.git
fi

cd homebrew-mxlogger

# 创建 Formula 目录
mkdir -p Formula

# 复制 formula
echo "📋 复制 formula 文件..."
cp ../Formula/mxlogger-analyzer-rust.rb ./Formula/

# 提交并推送
echo "📤 提交并推送到 GitHub..."
git add .
git commit -m "Add mxlogger-analyzer-rust v0.1.0"
git push origin main

echo "✅ Homebrew Tap 设置完成！"
echo ""
echo "🎉 用户现在可以通过以下方式安装："
echo "  brew tap suyulin/mxlogger"
echo "  brew install mxlogger-analyzer-rust"
echo ""
echo "🧪 测试安装:"
echo "  brew tap suyulin/mxlogger"
echo "  brew install mxlogger-analyzer-rust"
