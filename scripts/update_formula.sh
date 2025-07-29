#!/bin/bash
# 更新 Homebrew formula 的 SHA256 脚本

set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

FORMULA_FILE="Formula/mxlogger-analyzer-rust.rb"

# 计算各平台的 SHA256
echo "正在计算 SHA256..."

# macOS aarch64
MACOS_AARCH64_URL="https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v${VERSION}/mxlogger_analyzer_rust-macos-aarch64.tar.gz"
MACOS_AARCH64_SHA256=$(curl -sL "$MACOS_AARCH64_URL" | shasum -a 256 | cut -d' ' -f1)

# macOS x86_64
MACOS_X86_64_URL="https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v${VERSION}/mxlogger_analyzer_rust-macos-x86_64.tar.gz"
MACOS_X86_64_SHA256=$(curl -sL "$MACOS_X86_64_URL" | shasum -a 256 | cut -d' ' -f1)

# Linux x86_64
LINUX_X86_64_URL="https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v${VERSION}/mxlogger_analyzer_rust-linux-x86_64.tar.gz"
LINUX_X86_64_SHA256=$(curl -sL "$LINUX_X86_64_URL" | shasum -a 256 | cut -d' ' -f1)

echo "macOS aarch64 SHA256: $MACOS_AARCH64_SHA256"
echo "macOS x86_64 SHA256: $MACOS_X86_64_SHA256"
echo "Linux x86_64 SHA256: $LINUX_X86_64_SHA256"

# 创建更新后的 formula
cat > "$FORMULA_FILE" << EOF
class MxloggerAnalyzerRust < Formula
  desc "High-performance Rust tool for decoding and analyzing encrypted log files"
  homepage "https://github.com/suyulin/mxlogger_analyzer_rust"
  version "$VERSION"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-macos-aarch64.tar.gz"
      sha256 "$MACOS_AARCH64_SHA256"
    else
      url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-macos-x86_64.tar.gz"
      sha256 "$MACOS_X86_64_SHA256"
    end
  elsif OS.linux?
    url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-linux-x86_64.tar.gz"
    sha256 "$LINUX_X86_64_SHA256"
  end

  def install
    bin.install "mxlogger_analyzer_rust"
  end

  test do
    # Test that the binary exists and can display help
    assert_match "Usage:", shell_output("#{bin}/mxlogger_analyzer_rust --help")
  end
end
EOF

echo "Formula 已更新: $FORMULA_FILE"
