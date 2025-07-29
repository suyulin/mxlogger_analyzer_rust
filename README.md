# MxLogger Analyzer Rust

A high-performance Rust-based tool for decoding and analyzing encrypted log files. This tool is designed to decrypt MxLogger format files using AES encryption and parse log data using FlatBuffers format.

## Features

- 🔐 **AES Decryption**: Supports AES decryption with configurable keys
- 📊 **FlatBuffers Parsing**: Efficiently parses log data using FlatBuffers format
- 🌍 **Timezone Support**: Flexible timezone formatting for timestamps
- ⚡ **Async Processing**: Built with Tokio for high-performance async operations
- 📝 **Verbose Logging**: Detailed progress reporting and debugging information
- 🛠️ **CLI Interface**: User-friendly command-line interface with comprehensive help
- 🖥️ **Cross-Platform**: Supports Windows, macOS, and Linux

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Windows | x86_64 | ✅ Supported |
| Windows | ARM64 | ✅ Supported |
| macOS | Intel (x86_64) | ✅ Supported |
| macOS | Apple Silicon (ARM64) | ✅ Supported |
| Linux | x86_64 | ✅ Supported |

## Installation

### Windows (One-Click Install)

#### PowerShell (推荐)
```powershell
# Run in PowerShell (as Administrator or regular user)
irm https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.ps1 | iex
```

#### Command Prompt (CMD)
```cmd
REM Download and run install.cmd
curl -L -o install.cmd https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.cmd
install.cmd
```

#### Manual PowerShell Install
```powershell
# Download and run the installer script
$scriptUrl = "https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install-windows.ps1"
Invoke-WebRequest -Uri $scriptUrl -OutFile install-windows.ps1
.\install-windows.ps1
```

#### Windows Manual Download
Visit the [Releases page](https://github.com/suyulin/mxlogger_analyzer_rust/releases) and download:
- `mxlogger_analyzer_rust-windows-x86_64.zip` for 64-bit Windows
- `mxlogger_analyzer_rust-windows-aarch64.zip` for ARM64 Windows

Extract the `.exe` file and add it to your PATH or place it in a directory that's already in your PATH.

### macOS/Linux

### Using Homebrew (Recommended)

```bash
# Direct install (no tap required)
brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust
```

### Download from GitHub Releases

Visit the [Releases page](https://github.com/suyulin/mxlogger_analyzer_rust/releases) to download pre-compiled binaries for your system:
- Linux: `mxlogger_analyzer_rust-linux-x86_64.tar.gz`
- macOS Intel: `mxlogger_analyzer_rust-macos-x86_64.tar.gz`  
- macOS Apple Silicon: `mxlogger_analyzer_rust-macos-aarch64.tar.gz`

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo package manager

### Build from Source

```bash
git clone https://github.com/suyulin/mxlogger_analyzer_rust.git
cd mxlogger_analyzer_rust
cargo build --release
```

The compiled binary will be available at `target/release/mxlogger_analyzer_rust`.

## Usage

### Environment Variables

Before using the tool, you need to set the encryption keys as environment variables:

#### Windows (PowerShell)
```powershell
$env:MXLOGGER_CRYPT_KEY = "your_key"
$env:MXLOGGER_IV_KEY = "your_key"
```

#### Windows (Command Prompt)
```cmd
set MXLOGGER_CRYPT_KEY=your_key
set MXLOGGER_IV_KEY=your_key
```

#### macOS/Linux (Bash/Zsh)
```bash
export MXLOGGER_CRYPT_KEY=your_key
export MXLOGGER_IV_KEY=your_key
```

**Note**: Both environment variables are required for decryption.

### Basic Usage

```bash
# Basic decoding
./mxlogger_analyzer_rust logfile.bin

# Windows
.\mxlogger_analyzer_rust.exe logfile.bin

# With timezone specification
./mxlogger_analyzer_rust logfile.bin --timezone Asia/Shanghai

# With custom output filename
./mxlogger_analyzer_rust logfile.bin -o custom_output

# Verbose mode for detailed progress
./mxlogger_analyzer_rust logfile.bin --verbose

# Combined options
./mxlogger_analyzer_rust logfile.bin -z UTC -o decoded_logs -v
```

### Command Line Options

```
USAGE:
    mxlogger_analyzer_rust [OPTIONS] <FILE>

ARGUMENTS:
    <FILE>    Path to the input log file to decode

OPTIONS:
    -z, --timezone <TIMEZONE>    Specify the timezone for timestamp formatting
    -o, --output <NAME>          Specify output file name (without extension)
    -v, --verbose                Enable verbose output
    -h, --help                   Print help
    -V, --version                Print version
```

### Examples

#### Windows (PowerShell)
```powershell
# Set environment variables
$env:MXLOGGER_CRYPT_KEY = "your_key"
$env:MXLOGGER_IV_KEY = "your_key"

# Decode with Shanghai timezone
.\mxlogger_analyzer_rust.exe logs\app.bin --timezone Asia/Shanghai

# Decode with verbose output and custom filename
.\mxlogger_analyzer_rust.exe logs\app.bin -o application_logs --verbose

# Decode with UTC timezone
.\mxlogger_analyzer_rust.exe logs\app.bin -z UTC -o decoded_logs -v
```

#### macOS/Linux (Bash)
```bash
# Set environment variables
export MXLOGGER_CRYPT_KEY=your_key
export MXLOGGER_IV_KEY=your_key

# Decode with Shanghai timezone
./mxlogger_analyzer_rust logs/app.bin --timezone Asia/Shanghai

# Decode with verbose output and custom filename
./mxlogger_analyzer_rust logs/app.bin -o application_logs --verbose

# Decode with UTC timezone
./mxlogger_analyzer_rust logs/app.bin -z UTC -o decoded_logs -v
```

## Supported Timezones

The tool supports standard timezone identifiers, including:

- `UTC`
- `Asia/Shanghai`
- `America/New_York`
- `Europe/London`
- `Asia/Tokyo`
- And many others...

## Output Format

The tool generates a `.txt` file containing the decoded log entries with properly formatted timestamps according to the specified timezone.

## Error Handling

The tool provides detailed error reporting:

- **Error Code 1**: Usually indicates data corruption or incorrect file format
- **Error Code 2**: Might be related to encryption key mismatch
- **Other errors**: Unknown error type, check file integrity

## Dependencies

- **aes**: AES encryption/decryption
- **cipher**: Cryptographic cipher traits
- **byteorder**: Reading/writing binary data
- **flatbuffers**: Efficient serialization library
- **tokio**: Async runtime
- **cfb-mode**: CFB mode for block ciphers
- **chrono & chrono-tz**: Date and time handling with timezone support
- **clap**: Command-line argument parsing

## Project Structure

```
src/
├── main.rs                     # Main application entry point
├── crypto/                     # Cryptographic operations
│   └── mod.rs
├── decoder/                    # Log decoding logic
│   └── mod.rs
├── logger/                     # Logging utilities
│   └── mod.rs
└── log_serialize_generated.rs  # FlatBuffers generated code
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Troubleshooting

### Common Issues

1. **"Crypt key not found" error**
   - Ensure `MXLOGGER_CRYPT_KEY` environment variable is set
   - Check that the key is exactly 20 characters long

2. **"IV key not found" error**
   - Ensure `MXLOGGER_IV_KEY` environment variable is set
   - Check that the IV key is exactly 20 characters long

3. **Decoding errors**
   - Verify that the input file is a valid MxLogger format file
   - Check that the encryption keys are correct
   - Ensure the file is not corrupted

4. **Windows Installation Issues**
   - If PowerShell execution is restricted, run: `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`
   - Ensure you have internet connectivity to download the installer
   - Try running PowerShell as Administrator if PATH updates fail

5. **Windows PATH Issues**
   - Restart your terminal after installation
   - Or manually add the install directory to your PATH
   - Use the full path to the executable: `C:\Users\YourName\.local\bin\mxlogger_analyzer_rust.exe`

### Debug Mode

Use the `--verbose` flag to get detailed information about the decoding process:

```bash
./mxlogger_analyzer_rust logfile.bin --verbose
```

This will show:
- File size information
- Environment variable status
- Progress updates
- Detailed error information

## Performance

The tool is optimized for performance:
- Async I/O operations for file handling
- Efficient memory usage
- Progress tracking for large files
- Optimized binary parsing

## Version History

- **v0.1.0**: Initial release with basic decoding functionality

---

For more information or support, please open an issue on the GitHub repository.
