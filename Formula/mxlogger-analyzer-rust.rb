class MxloggerAnalyzerRust < Formula
  desc "High-performance Rust tool for decoding and analyzing encrypted log files"
  homepage "https://github.com/suyulin/mxlogger_analyzer_rust"
  version "0.1.0"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-macos-aarch64.tar.gz"
      sha256 "263ed8aa58d3ad17f3fe131163635153262fb9c647782e45f22d28e67a03ed0d"
    else
      url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-macos-x86_64.tar.gz"
      sha256 "beaf64f3d6e277fedf446a4831ee024eb9850566504f11bdc56285419409da60"
    end
  elsif OS.linux?
    url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-linux-x86_64.tar.gz"
    sha256 "734db5f94f96f8c7ce57fac5a9b9fa893f6a3dc7fa24c27b5cdcc7e90ef28e55"
  end

  def install
    bin.install "mxlogger_analyzer_rust"
  end

  test do
    # Test that the binary exists and can display help
    assert_match "Usage:", shell_output("#{bin}/mxlogger_analyzer_rust --help")
  end
end
