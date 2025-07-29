#!/bin/bash
# 使用 GitHub API 自动创建 homebrew-mxlogger 仓库

set -e

echo "🚀 自动创建 homebrew-mxlogger 仓库并设置 Homebrew Tap..."

# 检查是否设置了 GitHub token
if [ -z "$GITHUB_TOKEN" ]; then
    echo "❌ 请设置 GITHUB_TOKEN 环境变量"
    echo "   1. 访问: https://github.com/settings/tokens"
    echo "   2. 创建 Personal Access Token (classic)"
    echo "   3. 勾选 'repo' 权限"
    echo "   4. 运行: export GITHUB_TOKEN=your_token"
    exit 1
fi

# 创建仓库
echo "📋 创建 homebrew-mxlogger 仓库..."
curl -X POST \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github.v3+json" \
  https://api.github.com/user/repos \
  -d '{
    "name": "homebrew-mxlogger",
    "description": "Homebrew tap for MxLogger tools",
    "private": false,
    "auto_init": false
  }'

echo ""
echo "⏳ 等待仓库创建完成..."
sleep 3

# 克隆并设置仓库
echo "📥 克隆并设置 tap 仓库..."
git clone https://github.com/suyulin/homebrew-mxlogger.git
cd homebrew-mxlogger

# 创建 Formula 目录和文件
mkdir Formula
cp ../Formula/mxlogger-analyzer-rust.rb ./Formula/

# 初始化并推送
git add .
git commit -m "Add mxlogger-analyzer-rust v0.1.0"
git push origin main

cd ..

echo "✅ Homebrew Tap 创建完成！"
echo ""
echo "🧪 测试安装:"
echo "  brew tap suyulin/mxlogger"
echo "  brew install mxlogger-analyzer-rust"
echo ""
echo "🎉 用户安装命令:"
echo "  brew tap suyulin/mxlogger"
echo "  brew install mxlogger-analyzer-rust"
