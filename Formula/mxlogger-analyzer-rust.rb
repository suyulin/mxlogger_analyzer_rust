class MxloggerAnalyzerRust < Formula
  desc "High-performance Rust tool for decoding and analyzing encrypted log files"
  homepage "https://github.com/suyulin/mxlogger_analyzer_rust"
  version "0.1.0"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  elsif OS.linux?
    url "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/v#{version}/mxlogger_analyzer_rust-linux-x86_64.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "mxlogger_analyzer_rust"
  end

  test do
    # Test that the binary exists and can display help
    assert_match "Usage:", shell_output("#{bin}/mxlogger_analyzer_rust --help")
  end
end
