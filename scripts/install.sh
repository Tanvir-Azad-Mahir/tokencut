#!/usr/bin/env bash
# Install a published tokencut release into ~/.local/bin (no sudo required).
set -euo pipefail

REPOSITORY="Tanvir-Azad-Mahir/tokencut"
VERSION="${1:-v0.1.0}"
INSTALL_DIR="${TOKENCUT_INSTALL_DIR:-${HOME}/.local/bin}"

if [[ ! "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]]; then
  echo "Invalid release version: $VERSION (expected v0.1.0 format)" >&2
  exit 2
fi

for command in curl tar mktemp chmod mkdir cp; do
  if ! command -v "$command" >/dev/null 2>&1; then
    echo "Missing required command: $command" >&2
    exit 1
  fi
done

case "$(uname -s)" in
  Linux) platform="linux" ;;
  Darwin) platform="macos" ;;
  *) echo "Unsupported operating system. Use install.ps1 on Windows." >&2; exit 1 ;;
esac

case "$(uname -m)" in
  x86_64|amd64) arch="x64" ;;
  arm64|aarch64)
    if [[ "$platform" == "macos" ]]; then arch="arm64";
    else echo "Linux ARM64 release is not currently provided." >&2; exit 1; fi ;;
  *) echo "Unsupported CPU architecture: $(uname -m)" >&2; exit 1 ;;
esac

filename="tokencut-${VERSION}-${platform}-${arch}.tar.gz"
base="https://github.com/${REPOSITORY}/releases/download/${VERSION}"
workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

printf 'Fetching %s...\n' "$filename"
curl --fail --location --silent --show-error --retry 2 \
  --output "$workdir/$filename" "$base/$filename" || {
  echo "Download failed. Confirm the release has published binaries: https://github.com/${REPOSITORY}/releases/tag/${VERSION}" >&2
  exit 1
}
curl --fail --location --silent --show-error --retry 2 \
  --output "$workdir/SHA256SUMS.txt" "$base/SHA256SUMS.txt" || {
  echo "Failed to download SHA256SUMS.txt; refusing unverified installation." >&2
  exit 1
}

expected="$(awk -v name="$filename" '$2 == name || $2 == "*" name { print $1 }' "$workdir/SHA256SUMS.txt")"
if [[ ! "$expected" =~ ^[[:xdigit:]]{64}$ ]]; then
  echo "Cannot find a valid SHA-256 checksum for $filename; refusing installation." >&2
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  actual="$(sha256sum "$workdir/$filename" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  actual="$(shasum -a 256 "$workdir/$filename" | awk '{print $1}')"
else
  echo "Neither sha256sum nor shasum is installed; refusing unverified installation." >&2
  exit 1
fi
if [[ "${actual,,}" != "${expected,,}" ]]; then
  echo "SHA-256 checksum mismatch; refusing installation." >&2
  exit 1
fi

mkdir "$workdir/extracted"
tar -xzf "$workdir/$filename" -C "$workdir/extracted" tokencut
if [[ ! -f "$workdir/extracted/tokencut" ]]; then
  echo "Release archive does not contain tokencut." >&2
  exit 1
fi
mkdir -p "$INSTALL_DIR"
cp "$workdir/extracted/tokencut" "$INSTALL_DIR/tokencut"
chmod 755 "$INSTALL_DIR/tokencut"
printf 'Installed tokencut %s at %s\n' "$VERSION" "$INSTALL_DIR/tokencut"
"$INSTALL_DIR/tokencut" --version

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) printf '\nAdd this directory to PATH to run tokencut anywhere:\n  %s\n' "$INSTALL_DIR" ;;
esac
