# MxLogger 分析器 (Rust版)

一个高性能的 Rust 工具，用于解码和分析加密的日志文件。该工具专为解密 MxLogger 格式文件而设计，使用 AES 加密并通过 FlatBuffers 格式解析日志数据。

## 功能特性

- 🔐 **AES 解密**: 支持可配置密钥的 AES 解密
- 📊 **FlatBuffers 解析**: 高效解析 FlatBuffers 格式的日志数据
- 🌍 **时区支持**: 灵活的时间戳时区格式化
- ⚡ **异步处理**: 基于 Tokio 构建，提供高性能异步操作
- 📝 **详细日志**: 详细的进度报告和调试信息
- 🛠️ **命令行界面**: 用户友好的命令行界面，提供全面的帮助
- 🖥️ **跨平台**: 支持 Windows、macOS 和 Linux

## 支持的平台

| 平台 | 架构 | 状态 |
|----------|-------------|--------|
| Windows | x86_64 | ✅ 支持 |
| Windows | ARM64 | ✅ 支持 |
| macOS | Intel (x86_64) | ✅ 支持 |
| macOS | Apple Silicon (ARM64) | ✅ 支持 |
| Linux | x86_64 | ✅ 支持 |

## 安装

### Windows (一键安装)

#### PowerShell (推荐)
```powershell
# 在 PowerShell 中运行（管理员或普通用户均可）
irm https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.ps1 | iex
```

#### 命令提示符 (CMD)
```cmd
REM 下载并运行 install.cmd
curl -L -o install.cmd https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.cmd
install.cmd
```

#### 手动 PowerShell 安装
```powershell
# 下载并运行安装脚本
$scriptUrl = "https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install-windows.ps1"
Invoke-WebRequest -Uri $scriptUrl -OutFile install-windows.ps1
.\install-windows.ps1
```

#### Windows 手动下载
访问 [Releases 页面](https://github.com/suyulin/mxlogger_analyzer_rust/releases) 下载：
- `mxlogger_analyzer_rust-windows-x86_64.zip` 适用于 64 位 Windows
- `mxlogger_analyzer_rust-windows-aarch64.zip` 适用于 ARM64 Windows

解压 `.exe` 文件并将其添加到 PATH 或放置在已在 PATH 中的目录。

### macOS/Linux

### 使用 Homebrew (推荐)

```bash
# 直接安装（无需添加 tap）
brew install suyulin/mxlogger_analyzer_rust/mxlogger-analyzer-rust
```

### 从 GitHub Releases 下载

访问 [Releases 页面](https://github.com/suyulin/mxlogger_analyzer_rust/releases) 下载适合您系统的预编译二进制文件：
- Linux: `mxlogger_analyzer_rust-linux-x86_64.tar.gz`
- macOS Intel: `mxlogger_analyzer_rust-macos-x86_64.tar.gz`  
- macOS Apple Silicon: `mxlogger_analyzer_rust-macos-aarch64.tar.gz`

### 前置条件

- Rust 1.70+ (2021 版本)
- Cargo 包管理器

### 从源码构建

```bash
git clone https://github.com/suyulin/mxlogger_analyzer_rust.git
cd mxlogger_analyzer_rust
cargo build --release
```

编译后的二进制文件将位于 `target/release/mxlogger_analyzer_rust`。

## 使用方法

### 环境变量

使用工具之前，需要设置加密密钥环境变量：

#### Windows (PowerShell)
```powershell
$env:MXLOGGER_CRYPT_KEY = "your_key"
$env:MXLOGGER_IV_KEY = "your_key"
```

#### Windows (命令提示符)
```cmd
set MXLOGGER_CRYPT_KEY=your_key
set MXLOGGER_IV_KEY=your_key
```

#### macOS/Linux (Bash/Zsh)
```bash
export MXLOGGER_CRYPT_KEY=your_key
export MXLOGGER_IV_KEY=your_key
```

**注意**: 解密需要两个环境变量都设置。

### 基本用法

```bash
# 基本解码
./mxlogger_analyzer_rust logfile.bin

# Windows
.\mxlogger_analyzer_rust.exe logfile.bin

# 指定时区
./mxlogger_analyzer_rust logfile.bin --timezone Asia/Shanghai

# 自定义输出文件名
./mxlogger_analyzer_rust logfile.bin -o custom_output

# 详细模式显示进度
./mxlogger_analyzer_rust logfile.bin --verbose

# 组合选项
./mxlogger_analyzer_rust logfile.bin -z UTC -o decoded_logs -v
```

### 命令行选项

```
用法:
    mxlogger_analyzer_rust [选项] <文件>

参数:
    <文件>    要解码的输入日志文件路径

选项:
    -z, --timezone <时区>        指定时间戳格式化的时区
    -o, --output <名称>          指定输出文件名(不含扩展名)
    -v, --verbose               启用详细输出
    -h, --help                  显示帮助信息
    -V, --version               显示版本信息
```

### 使用示例

#### Windows (PowerShell)
```powershell
# 设置环境变量
$env:MXLOGGER_CRYPT_KEY = "your_key"
$env:MXLOGGER_IV_KEY = "your_key"

# 使用上海时区解码
.\mxlogger_analyzer_rust.exe logs\app.bin --timezone Asia/Shanghai

# 使用详细输出和自定义文件名解码
.\mxlogger_analyzer_rust.exe logs\app.bin -o application_logs --verbose

# 使用 UTC 时区解码
.\mxlogger_analyzer_rust.exe logs\app.bin -z UTC -o decoded_logs -v
```

#### macOS/Linux (Bash)
```bash
# 设置环境变量
export MXLOGGER_CRYPT_KEY=your_key
export MXLOGGER_IV_KEY=your_key

# 使用上海时区解码
./mxlogger_analyzer_rust logs/app.bin --timezone Asia/Shanghai

# 使用详细输出和自定义文件名解码
./mxlogger_analyzer_rust logs/app.bin -o application_logs --verbose

# 使用 UTC 时区解码
./mxlogger_analyzer_rust logs/app.bin -z UTC -o decoded_logs -v
```

## 支持的时区

该工具支持标准时区标识符，包括：

- `UTC`
- `Asia/Shanghai`
- `Asia/Beijing`
- `America/New_York`
- `Europe/London`
- `Asia/Tokyo`
- 以及其他更多时区...

## 输出格式

该工具生成包含解码日志条目的 `.txt` 文件，根据指定时区正确格式化时间戳。

## 错误处理

该工具提供详细的错误报告：

- **错误代码 1**: 通常表示数据损坏或文件格式不正确
- **错误代码 2**: 可能与加密密钥不匹配有关
- **其他错误**: 未知错误类型，请检查文件完整性

## 依赖项

- **aes**: AES 加密/解密
- **cipher**: 加密算法特征
- **byteorder**: 读写二进制数据
- **flatbuffers**: 高效序列化库
- **tokio**: 异步运行时
- **cfb-mode**: 块密码的 CFB 模式
- **chrono & chrono-tz**: 日期时间处理和时区支持
- **clap**: 命令行参数解析

## 项目结构

```
src/
├── main.rs                     # 主应用程序入口点
├── crypto/                     # 加密操作
│   └── mod.rs
├── decoder/                    # 日志解码逻辑
│   └── mod.rs
├── logger/                     # 日志工具
│   └── mod.rs
└── log_serialize_generated.rs  # FlatBuffers 生成的代码
```

## 贡献

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m '添加一些很棒的功能'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详情请查看 [LICENSE](LICENSE) 文件。

## 故障排除

### 常见问题

1. **"未找到加密密钥"错误**
   - 确保设置了 `MXLOGGER_CRYPT_KEY` 环境变量
   - 检查密钥长度是否正确

2. **"未找到 IV 密钥"错误**
   - 确保设置了 `MXLOGGER_IV_KEY` 环境变量
   - 检查 IV 密钥长度是否正确

3. **解码错误**
   - 验证输入文件是否为有效的 MxLogger 格式文件
   - 检查加密密钥是否正确
   - 确保文件未损坏

4. **Windows 安装问题**
   - 如果 PowerShell 执行受限，运行：`Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`
   - 确保有网络连接以下载安装程序
   - 如果 PATH 更新失败，尝试以管理员身份运行 PowerShell

5. **Windows PATH 问题**
   - 安装后重启终端
   - 或手动将安装目录添加到 PATH
   - 使用可执行文件的完整路径：`C:\Users\YourName\.local\bin\mxlogger_analyzer_rust.exe`

### 调试模式

使用 `--verbose` 标志获取解码过程的详细信息：

```bash
./mxlogger_analyzer_rust logfile.bin --verbose
```

这将显示：
- 文件大小信息
- 环境变量状态
- 进度更新
- 详细错误信息

## 性能

该工具针对性能进行了优化：
- 文件处理的异步 I/O 操作
- 高效的内存使用
- 大文件的进度跟踪
- 优化的二进制解析

## 版本历史

- **v0.1.0**: 初始版本，具备基本解码功能

---

如需更多信息或支持，请在 GitHub 仓库中创建 issue。
