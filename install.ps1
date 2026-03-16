# piz installer for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/AriesOxO/piz/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$repo = "AriesOxO/piz"
$target = "x86_64-pc-windows-msvc"
$installDir = "$env:LOCALAPPDATA\piz\bin"

function Write-Info($msg) { Write-Host "[info] $msg" -ForegroundColor Green }
function Write-Err($msg) { Write-Host "[error] $msg" -ForegroundColor Red; exit 1 }

Write-Host ""
Write-Host "  piz installer"
Write-Host "  -------------"
Write-Host ""

# Get latest release
Write-Info "Fetching latest version..."
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"
    $version = $release.tag_name
} catch {
    Write-Err "Could not determine latest version. Check https://github.com/$repo/releases"
}
Write-Info "Latest version: $version"

# Check for MSI first
$msiUrl = "https://github.com/$repo/releases/download/$version/piz-$target.msi"
$zipUrl = "https://github.com/$repo/releases/download/$version/piz-$target.zip"

# Try MSI install
$useMsi = Read-Host "Install via MSI installer? (recommended) [Y/n]"
if ($useMsi -ne "n" -and $useMsi -ne "N") {
    $tmpMsi = Join-Path $env:TEMP "piz-installer.msi"
    Write-Info "Downloading MSI..."
    try {
        Invoke-WebRequest -Uri $msiUrl -OutFile $tmpMsi -UseBasicParsing
        Write-Info "Launching MSI installer..."
        Start-Process msiexec.exe -ArgumentList "/i `"$tmpMsi`"" -Wait
        Remove-Item $tmpMsi -ErrorAction SilentlyContinue
        Write-Host ""
        Write-Info "piz installed via MSI. Restart your terminal to use it."
        exit 0
    } catch {
        Write-Host "[warn] MSI not available, falling back to zip install..." -ForegroundColor Yellow
    }
}

# Zip fallback
Write-Info "Downloading zip..."
$tmpZip = Join-Path $env:TEMP "piz.zip"
$tmpDir = Join-Path $env:TEMP "piz-extract"

try {
    Invoke-WebRequest -Uri $zipUrl -OutFile $tmpZip -UseBasicParsing
} catch {
    Write-Err "Download failed. Check if release exists for $target"
}

# Extract
if (Test-Path $tmpDir) { Remove-Item $tmpDir -Recurse -Force }
Expand-Archive -Path $tmpZip -DestinationPath $tmpDir -Force

# Install
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}
Copy-Item "$tmpDir\piz.exe" "$installDir\piz.exe" -Force

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
    Write-Info "Added $installDir to user PATH"
}

# Cleanup
Remove-Item $tmpZip -ErrorAction SilentlyContinue
Remove-Item $tmpDir -Recurse -ErrorAction SilentlyContinue

Write-Host ""
Write-Info "piz $version installed to $installDir\piz.exe"
Write-Host ""
Write-Host "  Restart your terminal, then:"
Write-Host "    piz --version"
Write-Host "    piz config --init"
Write-Host "    piz list files"
Write-Host ""
