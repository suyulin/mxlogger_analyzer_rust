# MxLogger Analyzer

[中文文档](README_zh.md)

A cross-platform command-line tool written in Rust for decoding MxLogger binary
logs. It supports AES-128-CFB decryption, FlatBuffers parsing, timezone-aware
timestamps, and progress output for troubleshooting.

## Features

- Decodes a single MxLogger file, a directory, or a decrypted app log ZIP
- Merges records from multiple files in timestamp order
- Writes readable `.log` output and optional structured JSONL
- Decrypts AES-128-CFB encrypted logs using environment-provided keys
- Parses FlatBuffers log records
- Formats timestamps with IANA timezones such as `Asia/Shanghai` and `UTC`
- Runs on macOS, Linux, and Windows

## Quick start

```bash
# Install on macOS or Linux
brew install suyulin/mxlogger/mxlogger-analyzer-rust

# Required when the input log is encrypted
export MXLOGGER_CRYPT_KEY="your-crypt-key"
export MXLOGGER_IV_KEY="your-iv-key"

# Decode an exported ZIP into app_logs.log and app_logs.jsonl
mxlogger_analyzer_rust app_logs.zip --timezone Asia/Shanghai --jsonl
```

## Installation

### Homebrew (macOS and Linux)

```bash
brew install suyulin/mxlogger/mxlogger-analyzer-rust
```

The tap name is `suyulin/mxlogger`. Homebrew resolves it to the
[`suyulin/homebrew-mxlogger`](https://github.com/suyulin/homebrew-mxlogger)
repository.

### Windows

Run the installer in PowerShell:

```powershell
irm https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.ps1 | iex
```

The binary is installed to `%USERPROFILE%\.local\bin`. Restart the terminal if
the command is not immediately available on `PATH`.

### Prebuilt binaries

Download an archive from
[GitHub Releases](https://github.com/suyulin/mxlogger_analyzer_rust/releases):

| Platform | Archive |
| --- | --- |
| macOS Apple Silicon | `mxlogger_analyzer_rust-macos-aarch64.tar.gz` |
| macOS Intel | `mxlogger_analyzer_rust-macos-x86_64.tar.gz` |
| Linux x86_64 | `mxlogger_analyzer_rust-linux-x86_64.tar.gz` |
| Windows x86_64 | `mxlogger_analyzer_rust-windows-x86_64.zip` |
| Windows ARM64 | `mxlogger_analyzer_rust-windows-aarch64.zip` |

Extract the archive and place the executable in a directory on your `PATH`.

### Build from source

Install a current stable Rust toolchain, then run:

```bash
git clone https://github.com/suyulin/mxlogger_analyzer_rust.git
cd mxlogger_analyzer_rust
cargo build --release
```

The executable is created at `target/release/mxlogger_analyzer_rust` (with an
`.exe` suffix on Windows).

## Encryption keys

Set both variables before decoding an encrypted log:

```bash
export MXLOGGER_CRYPT_KEY="your-crypt-key"
export MXLOGGER_IV_KEY="your-iv-key"
```

PowerShell:

```powershell
$env:MXLOGGER_CRYPT_KEY = "your-crypt-key"
$env:MXLOGGER_IV_KEY = "your-iv-key"
```

The decoder uses the first 16 bytes of each value and zero-pads shorter values.
If either variable is missing, decryption is skipped; this is useful for
unencrypted MxLogger files.

Do not commit real encryption keys to source control or include them in bug
reports.

## Usage

```text
mxlogger_analyzer_rust [OPTIONS] <INPUT>

Arguments:
  <INPUT>  MxLogger file, directory, or decrypted ZIP bundle

Options:
  -z, --timezone <TIMEZONE>  IANA timezone for timestamps
  -o, --output <NAME>        Output path without the .log/.jsonl extension
      --jsonl                Also write structured JSON Lines output
  -v, --verbose              Show decoding details and progress
  -h, --help                 Print help
  -V, --version              Print version
```

Examples:

```bash
# Creates app.log in the current directory
mxlogger_analyzer_rust app.bin

# Creates decoded_session.log with UTC timestamps
mxlogger_analyzer_rust app.bin --timezone UTC --output decoded_session

# Show progress and decoding errors
mxlogger_analyzer_rust app.bin -z Asia/Shanghai -v

# Merge every .mx file in a directory and also create decoded.jsonl
mxlogger_analyzer_rust ./logs -o decoded --jsonl
```

PowerShell:

```powershell
mxlogger_analyzer_rust.exe .\logs\app.bin -z Asia/Shanghai -o decoded -v
```

## Output

The text decoder writes one escaped record per line, including logger name, tag,
thread details, and source file:

```text
2026-07-31 15:30:42.000000+08:00 INFO name=APP tag=startup thread_id=1 main_thread=true source=2026-07-31_app.mx application started
```

With `--jsonl`, each object contains `timestamp`, `level`, `name`, `tag`,
`thread_id`, `is_main_thread`, `message`, and `source_file`.

Without `--output`, the input file stem is used and the result is written to the
current directory. For example, `logs/app.bin` creates `app.log`. The default
timezone is `Asia/Shanghai`.

Timezone values must be valid
[IANA timezone identifiers](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones),
for example `UTC`, `Asia/Shanghai`, `America/New_York`, or `Europe/London`.

## Troubleshooting

- **Homebrew reports `homebrew-mxlogger_analyzer_rust` not found**: use
  `brew install suyulin/mxlogger/mxlogger-analyzer-rust`. The middle component
  is the tap name, not the project repository name.
- **The output contains parse errors**: verify that the input is an MxLogger
  binary log and that both encryption variables match the keys used to write it.
- **The command is not found on Windows**: restart the terminal or add
  `%USERPROFILE%\.local\bin` to `PATH`.
- **More diagnostics are needed**: rerun the command with `--verbose`.

## Development

```bash
cargo fmt --check
cargo test
cargo build --release
```

The FlatBuffers-generated Rust source is kept in
`src/log_serialize_generated.rs`; its schema is `log_serialize.fbs`.

## License

MIT License.
