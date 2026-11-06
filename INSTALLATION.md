# Installing tokencut

These scripts download the **published** v0.1.0 release binaries, validate each archive against `SHA256SUMS.txt`, and install the executable without administrator privileges. They do not build from source and do not change your PATH automatically.

## Linux x64 and macOS (Intel or Apple Silicon)

From a trusted copy of this repository, run:

```bash
bash scripts/install.sh
```

Or fetch and inspect the installer before running it:

```bash
curl -fsSLo install.sh https://raw.githubusercontent.com/Tanvir-Azad-Mahir/tokencut/main/scripts/install.sh
less install.sh
bash install.sh v0.1.0
```

The default destination is `~/.local/bin/tokencut`. For a custom destination:

```bash
TOKENCUT_INSTALL_DIR="$HOME/bin" bash scripts/install.sh v0.1.0
```

Ensure that directory is on your `PATH`. For Bash:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

To make that persistent, add the line to your `~/.bashrc` or equivalent shell configuration.

## Windows x64 (PowerShell)

From a trusted local checkout:

```powershell
.\scripts\install.ps1
```

Or download and inspect first:

```powershell
Invoke-WebRequest https://raw.githubusercontent.com/Tanvir-Azad-Mahir/tokencut/main/scripts/install.ps1 -OutFile install.ps1
Get-Content .\install.ps1
.\install.ps1 -Version v0.1.0
```

If your execution policy blocks scripts, consult your environment's policy before changing it. Avoid disabling system-wide protections solely for installation.

The default destination is `$HOME\.local\bin\tokencut.exe`. The installer prints a note if this directory isn't on `PATH`. You can add it to your **user** PATH through Windows Environment Variables.

## Confirm it works

```bash
printf 'retry\nretry\nfailed\n' | tokencut
```

PowerShell:

```powershell
"retry`nretry`nfailed" | tokencut
```

## Important prerequisites

- The GitHub Release must contain archives and `SHA256SUMS.txt`. A GitHub Release page without attached assets is **not** installable through these scripts.
- Supported release targets: Linux x64, Windows x64, macOS Intel x64, macOS Apple Silicon arm64. Linux arm64 is not provided by the current workflow.
- Checksums establish integrity against the release's checksum file, not independently verified provenance or a digital signature.
- If you want a different version, pass it as an argument: `bash scripts/install.sh v0.1.1` or `./scripts/install.ps1 -Version v0.1.1`.
- Update the default version in the scripts when publishing a new release; both default to `v0.1.0` to accommodate a pre-release that is not returned by GitHub's “latest stable” endpoint.
