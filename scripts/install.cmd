@echo off
REM MxLogger Analyzer Rust - Windows One-Click Installer (CMD Version)
REM This batch file downloads and runs the PowerShell installer

echo.
echo 🚀 MxLogger Analyzer Rust - One-Click Installer
echo ============================================
echo.

REM Check if PowerShell is available
powershell -Command "Write-Host 'PowerShell is available'" >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ❌ PowerShell is required but not available.
    echo Please install PowerShell or use manual installation.
    echo.
    echo Manual installation:
    echo 1. Visit: https://github.com/suyulin/mxlogger_analyzer_rust/releases
    echo 2. Download the Windows .zip file for your architecture
    echo 3. Extract and add to PATH
    pause
    exit /b 1
)

echo 📥 Downloading and running PowerShell installer...
echo.

REM Download and execute the PowerShell installer
powershell -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install.ps1 | iex"

if %ERRORLEVEL% equ 0 (
    echo.
    echo ✅ Installation completed successfully!
    echo.
    echo 💡 You may need to restart your command prompt for PATH changes to take effect.
    echo.
) else (
    echo.
    echo ❌ Installation failed. Please try manual installation:
    echo 1. Visit: https://github.com/suyulin/mxlogger_analyzer_rust/releases
    echo 2. Download the Windows .zip file for your architecture
    echo 3. Extract and add to PATH
    echo.
)

pause