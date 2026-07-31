# MxLogger Analyzer

[English](README.md)

一个使用 Rust 编写的跨平台命令行工具，用于解码 MxLogger 二进制日志。支持
AES-128-CFB 解密、FlatBuffers 解析、时区格式化以及详细的处理进度输出。

## 功能

- 支持单个 MxLogger 文件、目录或已解密的 App 日志 ZIP
- 将多个文件中的记录按时间戳合并排序
- 输出可读 `.log`，并可选输出结构化 JSONL
- 通过环境变量提供密钥，解密 AES-128-CFB 加密日志
- 解析 FlatBuffers 日志记录
- 使用 `Asia/Shanghai`、`UTC` 等 IANA 时区格式化时间戳
- 支持 macOS、Linux 和 Windows

## 快速开始

```bash
# 在 macOS 或 Linux 上安装
brew install suyulin/mxlogger/mxlogger-analyzer-rust

# 输入文件已加密时需要设置
export MXLOGGER_CRYPT_KEY="your-crypt-key"
export MXLOGGER_IV_KEY="your-iv-key"

# 将导出的 ZIP 解码为 app_logs.log 和 app_logs.jsonl
mxlogger_analyzer_rust app_logs.zip --timezone Asia/Shanghai --jsonl
```

## 安装

### Homebrew（macOS 和 Linux）

```bash
brew install suyulin/mxlogger/mxlogger-analyzer-rust
```

Tap 名称是 `suyulin/mxlogger`，Homebrew 会将其解析到
[`suyulin/homebrew-mxlogger`](https://github.com/suyulin/homebrew-mxlogger)
仓库。

### Windows

在 PowerShell 中运行：

```powershell
irm https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.ps1 | iex
```

程序默认安装到 `%USERPROFILE%\.local\bin`。如果安装后命令暂时不可用，请重启终端，
使 `PATH` 变更生效。

### 下载预编译程序

从 [GitHub Releases](https://github.com/suyulin/mxlogger_analyzer_rust/releases)
下载对应压缩包：

| 平台 | 压缩包 |
| --- | --- |
| macOS Apple Silicon | `mxlogger_analyzer_rust-macos-aarch64.tar.gz` |
| macOS Intel | `mxlogger_analyzer_rust-macos-x86_64.tar.gz` |
| Linux x86_64 | `mxlogger_analyzer_rust-linux-x86_64.tar.gz` |
| Windows x86_64 | `mxlogger_analyzer_rust-windows-x86_64.zip` |
| Windows ARM64 | `mxlogger_analyzer_rust-windows-aarch64.zip` |

解压后，将可执行文件放入 `PATH` 包含的目录。

### 从源码构建

安装当前稳定版 Rust 工具链，然后运行：

```bash
git clone https://github.com/suyulin/mxlogger_analyzer_rust.git
cd mxlogger_analyzer_rust
cargo build --release
```

可执行文件位于 `target/release/mxlogger_analyzer_rust`，Windows 上带有 `.exe`
扩展名。

## 加密密钥

解码加密日志前设置以下两个变量：

```bash
export MXLOGGER_CRYPT_KEY="your-crypt-key"
export MXLOGGER_IV_KEY="your-iv-key"
```

PowerShell：

```powershell
$env:MXLOGGER_CRYPT_KEY = "your-crypt-key"
$env:MXLOGGER_IV_KEY = "your-iv-key"
```

解码器取每个变量的前 16 字节；不足 16 字节时使用零补齐。如果缺少任意一个变量，
程序将跳过解密，这可用于处理未加密的 MxLogger 文件。

请勿将真实密钥提交到代码仓库或粘贴到 Issue 中。

## 使用方法

```text
mxlogger_analyzer_rust [选项] <输入>

参数：
  <输入>  MxLogger 文件、目录或已解密的 ZIP

选项：
  -z, --timezone <时区>  时间戳使用的 IANA 时区
  -o, --output <名称>    输出路径，不包含 .log/.jsonl 扩展名
      --jsonl            同时生成结构化 JSON Lines
  -v, --verbose          显示解码详情和进度
  -h, --help             显示帮助
  -V, --version          显示版本
```

示例：

```bash
# 在当前目录生成 app.log
mxlogger_analyzer_rust app.bin

# 使用 UTC 时间，生成 decoded_session.log
mxlogger_analyzer_rust app.bin --timezone UTC --output decoded_session

# 显示进度和解码错误
mxlogger_analyzer_rust app.bin -z Asia/Shanghai -v

# 合并目录中的全部 .mx 文件，同时生成 decoded.jsonl
mxlogger_analyzer_rust ./logs -o decoded --jsonl
```

PowerShell：

```powershell
mxlogger_analyzer_rust.exe .\logs\app.bin -z Asia/Shanghai -o decoded -v
```

## 输出

文本输出每行是一条转义后的日志，包含 logger 名称、tag、线程和来源文件：

```text
2026-07-31 15:30:42.000000+08:00 INFO name=APP tag=startup thread_id=1 main_thread=true source=2026-07-31_app.mx application started
```

使用 `--jsonl` 时，每个对象固定包含 `timestamp`、`level`、`name`、`tag`、
`thread_id`、`is_main_thread`、`message` 和 `source_file`。

未设置 `--output` 时，程序使用输入文件名（不含扩展名），并将结果写入当前目录。
例如，输入 `logs/app.bin` 会生成 `app.log`。默认时区为 `Asia/Shanghai`。

时区必须是有效的
[IANA 时区标识符](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)，
例如 `UTC`、`Asia/Shanghai`、`America/New_York` 或 `Europe/London`。

## 常见问题

- **Homebrew 提示找不到 `homebrew-mxlogger_analyzer_rust`**：请使用
  `brew install suyulin/mxlogger/mxlogger-analyzer-rust`。中间部分是 tap 名称，
  不是项目仓库名。
- **输出中出现解析错误**：确认输入是 MxLogger 二进制日志，并确认两个环境变量与
  写入日志时使用的密钥一致。
- **Windows 安装后找不到命令**：重启终端，或将
  `%USERPROFILE%\.local\bin` 添加到 `PATH`。
- **需要更多诊断信息**：添加 `--verbose` 参数重新运行。

## 开发

```bash
cargo fmt --check
cargo test
cargo build --release
```

FlatBuffers 生成的 Rust 源码位于 `src/log_serialize_generated.rs`，对应 schema
为 `log_serialize.fbs`。

## 许可证

MIT License。
