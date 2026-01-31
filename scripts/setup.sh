#!/usr/bin/env bash
set -euo pipefail

PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
BIN_DIR="${PLUGIN_ROOT}/bin"
BINARY_NAME="ralph-hook-fmt"
BINARY_PATH="${BIN_DIR}/${BINARY_NAME}"

# Check if binary already exists and is executable
if [[ -x "${BINARY_PATH}" ]]; then
    exit 0
fi

# Determine OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "${OS}" in
    darwin)
        case "${ARCH}" in
            arm64) TARGET="aarch64-apple-darwin" ;;
            x86_64) TARGET="x86_64-apple-darwin" ;;
            *) echo "Unsupported architecture: ${ARCH}" >&2; exit 1 ;;
        esac
        ;;
    linux)
        case "${ARCH}" in
            aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
            x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
            *) echo "Unsupported architecture: ${ARCH}" >&2; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: ${OS}" >&2
        exit 1
        ;;
esac

# Get the latest release version from GitHub
REPO="chenhunghan/ralph-hook-fmt"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [[ -z "${LATEST_RELEASE}" ]]; then
    echo "Failed to fetch latest release version" >&2
    exit 1
fi

# Download URL
TARBALL_NAME="${BINARY_NAME}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_RELEASE}/${TARBALL_NAME}"

# Create bin directory
mkdir -p "${BIN_DIR}"

# Download and extract
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "${TEMP_DIR}"' EXIT

echo "Downloading ${BINARY_NAME} ${LATEST_RELEASE} for ${TARGET}..."
curl -sL "${DOWNLOAD_URL}" -o "${TEMP_DIR}/${TARBALL_NAME}"

echo "Extracting..."
tar -xzf "${TEMP_DIR}/${TARBALL_NAME}" -C "${TEMP_DIR}"

# Move binary to bin directory
mv "${TEMP_DIR}/${BINARY_NAME}" "${BINARY_PATH}"
chmod +x "${BINARY_PATH}"

echo "${BINARY_NAME} ${LATEST_RELEASE} installed successfully"
