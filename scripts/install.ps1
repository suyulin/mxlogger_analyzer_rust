# MxLogger Analyzer Rust - One-Click Windows Installer
# Run this in PowerShell as Administrator or regular user

# Download and execute the installer script
$scriptUrl = "https://raw.githubusercontent.com/suyulin/mxlogger_analyzer_rust/main/scripts/install-windows.ps1"

Write-Host "🚀 MxLogger Analyzer Rust - One-Click Installer" -ForegroundColor Green
Write-Host "Downloading installer script..." -ForegroundColor Blue

try {
    $script = Invoke-WebRequest -Uri $scriptUrl -UseBasicParsing | Select-Object -ExpandProperty Content
    Invoke-Expression $script
} catch {
    Write-Host "❌ Failed to download or execute installer: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "" -ForegroundColor White
    Write-Host "Manual installation options:" -ForegroundColor Yellow
    Write-Host "1. Download from: https://github.com/suyulin/mxlogger_analyzer_rust/releases" -ForegroundColor White
    Write-Host "2. Or run:" -ForegroundColor White
    Write-Host "   irm $scriptUrl | iex" -ForegroundColor White
    exit 1
}