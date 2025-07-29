#!/bin/bash
# 自动发布脚本

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查参数
if [ $# -ne 1 ]; then
    echo -e "${RED}Usage: $0 <version>${NC}"
    echo -e "${YELLOW}Example: $0 0.1.0${NC}"
    exit 1
fi

VERSION=$1
TAG="v$VERSION"

echo -e "${GREEN}开始发布 MxLogger Analyzer Rust v$VERSION${NC}"

# 检查是否在正确的目录
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}错误: 请在项目根目录运行此脚本${NC}"
    exit 1
fi

# 检查是否有未提交的更改
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}错误: 存在未提交的更改，请先提交所有更改${NC}"
    exit 1
fi

# 更新 Cargo.toml 中的版本号
echo -e "${YELLOW}更新 Cargo.toml 中的版本号...${NC}"
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# 检查 Cargo.toml 是否更新成功
if ! grep -q "version = \"$VERSION\"" Cargo.toml; then
    echo -e "${RED}错误: 无法更新 Cargo.toml 中的版本号${NC}"
    exit 1
fi

echo -e "${GREEN}版本号已更新为 $VERSION${NC}"

# 构建项目以确保没有编译错误
echo -e "${YELLOW}构建项目...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}错误: 构建失败${NC}"
    exit 1
fi

echo -e "${GREEN}构建成功${NC}"

# 运行测试
echo -e "${YELLOW}运行测试...${NC}"
cargo test

if [ $? -ne 0 ]; then
    echo -e "${RED}错误: 测试失败${NC}"
    exit 1
fi

echo -e "${GREEN}测试通过${NC}"

# 提交版本更改
echo -e "${YELLOW}提交版本更改...${NC}"
git add Cargo.toml
git commit -m "Bump version to $VERSION"

# 创建并推送标签
echo -e "${YELLOW}创建标签 $TAG...${NC}"
git tag "$TAG"

echo -e "${YELLOW}推送到远程仓库...${NC}"
git push origin main
git push origin "$TAG"

echo -e "${GREEN}标签 $TAG 已推送，GitHub Actions 将开始构建...${NC}"

# 等待用户确认 GitHub Actions 完成
echo -e "${YELLOW}请在 GitHub 上检查 Actions 是否成功完成，然后按任意键继续...${NC}"
read -n 1 -s

# 更新 Homebrew formula
echo -e "${YELLOW}更新 Homebrew formula...${NC}"
./scripts/update_formula.sh "$VERSION"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Homebrew formula 已更新${NC}"
    
    # 提交 formula 更改
    git add Formula/mxlogger-analyzer-rust.rb
    git commit -m "Update Homebrew formula for v$VERSION"
    git push origin main
    
    echo -e "${GREEN}发布完成！${NC}"
    echo -e "${YELLOW}下一步:${NC}"
    echo -e "1. 检查 GitHub Release 页面: https://github.com/suyulin/mxlogger_analyzer_rust/releases"
    echo -e "2. 如果使用个人 tap，将 formula 推送到 homebrew-mxlogger 仓库"
    echo -e "3. 测试安装: brew tap suyulin/mxlogger && brew install mxlogger-analyzer-rust"
else
    echo -e "${RED}警告: Homebrew formula 更新失败，请手动更新${NC}"
fi
