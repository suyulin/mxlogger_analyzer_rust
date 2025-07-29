# MxLogger Analyzer Rust - Windows Installation Script
# This script downloads and installs the latest release of mxlogger_analyzer_rust for Windows

param(
    [string]$InstallPath = "$env:USERPROFILE\.local\bin",
    [string]$Version = "latest",
    [switch]$Force = $false
)

$ErrorActionPreference = "Stop"

# Colors for output
$Red = [ConsoleColor]::Red
$Green = [ConsoleColor]::Green
$Yellow = [ConsoleColor]::Yellow
$Blue = [ConsoleColor]::Blue

function Write-ColorOutput {
    param([string]$Message, [ConsoleColor]$Color = [ConsoleColor]::White)
    Write-Host $Message -ForegroundColor $Color
}

function Test-AdminRights {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-Architecture {
    if ([Environment]::Is64BitOperatingSystem) {
        if ([Environment]::Is64BitProcess) {
            return "x86_64"
        } else {
            return "x86_64"  # Still use x86_64 binary even on 32-bit process
        }
    } else {
        throw "32-bit Windows is not supported"
    }
}

function Get-LatestRelease {
    Write-ColorOutput "🔍 Fetching latest release information..." $Blue
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/suyulin/mxlogger_analyzer_rust/releases/latest" -Headers @{"User-Agent" = "mxlogger-installer"}
        return $response
    } catch {
        throw "Failed to fetch release information: $($_.Exception.Message)"
    }
}

function Download-File {
    param([string]$Url, [string]$OutputPath)
    
    Write-ColorOutput "📥 Downloading from: $Url" $Blue
    try {
        $webClient = New-Object System.Net.WebClient
        $webClient.DownloadFile($Url, $OutputPath)
        Write-ColorOutput "✅ Downloaded successfully" $Green
    } catch {
        throw "Failed to download file: $($_.Exception.Message)"
    }
}

function Extract-Archive {
    param([string]$ArchivePath, [string]$ExtractPath)
    
    Write-ColorOutput "📦 Extracting archive..." $Blue
    try {
        Expand-Archive -Path $ArchivePath -DestinationPath $ExtractPath -Force
        Write-ColorOutput "✅ Extracted successfully" $Green
    } catch {
        throw "Failed to extract archive: $($_.Exception.Message)"
    }
}

function Add-ToPath {
    param([string]$Directory)
    
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -notlike "*$Directory*") {
        Write-ColorOutput "🔧 Adding to PATH: $Directory" $Yellow
        $newPath = "$currentPath;$Directory"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Write-ColorOutput "✅ Added to PATH (restart terminal to take effect)" $Green
        return $true
    } else {
        Write-ColorOutput "ℹ️  Directory already in PATH" $Blue
        return $false
    }
}

# Main installation logic
try {
    Write-ColorOutput "🚀 MxLogger Analyzer Rust - Windows Installer" $Green
    Write-ColorOutput "============================================" $Green

    # Check architecture
    $arch = Get-Architecture
    Write-ColorOutput "🖥️  Detected architecture: $arch" $Blue

    # Create install directory
    if (-not (Test-Path $InstallPath)) {
        Write-ColorOutput "📁 Creating install directory: $InstallPath" $Yellow
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }

    # Check if already installed
    $exePath = Join-Path $InstallPath "mxlogger_analyzer_rust.exe"
    if ((Test-Path $exePath) -and -not $Force) {
        Write-ColorOutput "⚠️  mxlogger_analyzer_rust is already installed at $exePath" $Yellow
        Write-ColorOutput "Use -Force to reinstall" $Yellow
        $response = Read-Host "Do you want to continue and reinstall? (y/N)"
        if ($response -ne "y" -and $response -ne "Y") {
            Write-ColorOutput "❌ Installation cancelled" $Red
            exit 1
        }
    }

    # Get release information
    if ($Version -eq "latest") {
        $release = Get-LatestRelease
        $Version = $release.tag_name
    }

    Write-ColorOutput "📋 Installing version: $Version" $Blue

    # Construct download URL
    $filename = "mxlogger_analyzer_rust-windows-$arch.zip"
    $downloadUrl = "https://github.com/suyulin/mxlogger_analyzer_rust/releases/download/$Version/$filename"

    # Create temporary directory
    $tempDir = Join-Path $env:TEMP "mxlogger_install_$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

    try {
        # Download the archive
        $archivePath = Join-Path $tempDir $filename
        Download-File -Url $downloadUrl -OutputPath $archivePath

        # Extract the archive
        $extractPath = Join-Path $tempDir "extract"
        Extract-Archive -ArchivePath $archivePath -ExtractPath $extractPath

        # Copy the executable
        $sourcePath = Join-Path $extractPath "mxlogger_analyzer_rust.exe"
        if (-not (Test-Path $sourcePath)) {
            throw "Executable not found in archive"
        }

        Write-ColorOutput "📋 Installing to: $exePath" $Blue
        Copy-Item $sourcePath $exePath -Force

        # Verify installation
        if (Test-Path $exePath) {
            Write-ColorOutput "✅ Installation successful!" $Green
            
            # Add to PATH if needed
            $pathUpdated = Add-ToPath -Directory $InstallPath
            
            # Test the installation
            Write-ColorOutput "🧪 Testing installation..." $Blue
            $version = & $exePath --version 2>&1
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✅ Installation test successful: $version" $Green
            } else {
                Write-ColorOutput "⚠️  Installation test failed, but binary was installed" $Yellow
            }

            Write-ColorOutput "" $White
            Write-ColorOutput "🎉 Installation completed!" $Green
            Write-ColorOutput "📍 Binary location: $exePath" $Blue
            
            if ($pathUpdated) {
                Write-ColorOutput "🔄 Please restart your terminal or run 'refreshenv' to use the 'mxlogger_analyzer_rust' command" $Yellow
            } else {
                Write-ColorOutput "💡 You can now use 'mxlogger_analyzer_rust' command from anywhere" $Blue
            }
            
            Write-ColorOutput "" $White
            Write-ColorOutput "📚 Next steps:" $Blue
            Write-ColorOutput "1. Set environment variables:" $White
            Write-ColorOutput "   `$env:MXLOGGER_CRYPT_KEY = 'your_key'" $White
            Write-ColorOutput "   `$env:MXLOGGER_IV_KEY = 'your_key'" $White
            Write-ColorOutput "2. Run: mxlogger_analyzer_rust --help" $White
            
        } else {
            throw "Failed to install executable"
        }

    } finally {
        # Clean up temporary files
        if (Test-Path $tempDir) {
            Remove-Item $tempDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

} catch {
    Write-ColorOutput "❌ Installation failed: $($_.Exception.Message)" $Red
    exit 1
}