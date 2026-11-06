# Install a published Windows x64 tokencut release without administrator privileges.
param(
    [string]$Version = 'v0.1.0',
    [string]$InstallDir = (Join-Path $HOME '.local\bin')
)

$ErrorActionPreference = 'Stop'
$Repository = 'Tanvir-Azad-Mahir/tokencut'

if ($Version -notmatch '^v\d+\.\d+\.\d+([.-][0-9A-Za-z.-]+)?$') {
    throw "Invalid release version: $Version (expected v0.1.0 format)"
}

if (-not [Environment]::Is64BitOperatingSystem -or [Environment]::GetEnvironmentVariable('PROCESSOR_ARCHITECTURE') -notin @('AMD64', 'x86_64')) {
    throw 'This installer supports Windows x64 only. Use a native x64 PowerShell session.'
}

$FileName = "tokencut-$Version-windows-x64.zip"
$Base = "https://github.com/$Repository/releases/download/$Version"
$WorkDir = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $WorkDir -Force | Out-Null

try {
    $Archive = Join-Path $WorkDir $FileName
    $Checksums = Join-Path $WorkDir 'SHA256SUMS.txt'
    Write-Host "Fetching $FileName..."
    try {
        Invoke-WebRequest -Uri "$Base/$FileName" -OutFile $Archive -UseBasicParsing
        Invoke-WebRequest -Uri "$Base/SHA256SUMS.txt" -OutFile $Checksums -UseBasicParsing
    } catch {
        throw "Release assets are not yet downloadable. Check https://github.com/$Repository/releases/tag/$Version. $($_.Exception.Message)"
    }

    $Expected = $null
    foreach ($Line in (Get-Content -LiteralPath $Checksums)) {
        if ($Line -match '^([0-9a-fA-F]{64})\s+\*?(.+)$' -and $Matches[2] -eq $FileName) {
            $Expected = $Matches[1]
            break
        }
    }
    if (-not $Expected) { throw "No SHA-256 checksum for $FileName; refusing installation." }
    $Actual = (Get-FileHash -LiteralPath $Archive -Algorithm SHA256).Hash
    if ($Actual -ine $Expected) { throw 'SHA-256 checksum mismatch; refusing installation.' }

    $Extracted = Join-Path $WorkDir 'extracted'
    Expand-Archive -LiteralPath $Archive -DestinationPath $Extracted -Force
    $Exe = Join-Path $Extracted 'tokencut.exe'
    if (-not (Test-Path -LiteralPath $Exe -PathType Leaf)) {
        throw 'Release archive does not contain tokencut.exe.'
    }
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    $Target = Join-Path $InstallDir 'tokencut.exe'
    Copy-Item -LiteralPath $Exe -Destination $Target -Force
    Write-Host "Installed tokencut $Version at $Target"
    & $Target --version

    $Paths = $env:PATH -split ';'
    if ($InstallDir -notin $Paths) {
        Write-Host "Add $InstallDir to your user PATH to run tokencut from any terminal."
    }
} finally {
    Remove-Item -LiteralPath $WorkDir -Recurse -Force -ErrorAction SilentlyContinue
}
