#!/bin/bash
# ccguilt installer
# curl -sSL https://raw.githubusercontent.com/aayushh-code/ccguilt/master/install.sh | bash
set -e

REPO="aayushh-code/ccguilt"
VERSION="latest"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="ccguilt"

echo "==================================================================="
echo "  CCGUILT INSTALLER"
echo "  Claude Code Guilt Trip — because the planet wasn't suffering enough"
echo "==================================================================="
echo ""

# Detect OS
OS=$(uname -s)
case "$OS" in
    Linux)  OS_TAG="linux" ;;
    Darwin) OS_TAG="macos" ;;
    *)
        echo "Error: Unsupported OS: $OS"
        exit 1
        ;;
esac

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64)   ARCH_TAG="x86_64" ;;
    aarch64|arm64)   ARCH_TAG="aarch64" ;;
    *)
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

ASSET_NAME="ccguilt-${OS_TAG}-${ARCH_TAG}"

# Get latest release download URL
if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"
else
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"
fi

echo "Detected: ${OS} ${ARCH}"
echo "Downloading ${ASSET_NAME}..."
echo ""

TMP=$(mktemp)
if command -v curl &>/dev/null; then
    curl -sSL -o "$TMP" "$DOWNLOAD_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$TMP" "$DOWNLOAD_URL"
else
    echo "Error: curl or wget required"
    exit 1
fi

chmod +x "$TMP"

# Verify it's a real binary
if ! file "$TMP" | grep -q "executable"; then
    echo "Error: Downloaded file is not a valid binary. Release may not exist yet."
    echo "Check: https://github.com/${REPO}/releases"
    rm -f "$TMP"
    exit 1
fi

# Install
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP" "${INSTALL_DIR}/${BINARY_NAME}"
    echo "Installed to ${INSTALL_DIR}/${BINARY_NAME}"
elif command -v sudo &>/dev/null; then
    echo "Installing to ${INSTALL_DIR} (requires sudo)..."
    sudo mv "$TMP" "${INSTALL_DIR}/${BINARY_NAME}"
    sudo chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    echo "Installed to ${INSTALL_DIR}/${BINARY_NAME}"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    mv "$TMP" "${INSTALL_DIR}/${BINARY_NAME}"
    echo "Installed to ${INSTALL_DIR}/${BINARY_NAME}"
    echo ""
    echo "Make sure ${INSTALL_DIR} is in your PATH:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""
echo "Done! Run 'ccguilt --version' to verify."
echo "Run 'ccguilt daily' to see your environmental destruction."

# Auto-register MCP server if Claude Code is installed
if command -v claude &>/dev/null; then
    echo ""
    echo "Detected Claude Code — registering ccguilt MCP server..."
    if "${INSTALL_DIR}/${BINARY_NAME}" --setup-mcp 2>&1; then
        echo "  Open a new Claude Code session and try: \"how much CO2 have I burned today?\""
    else
        echo "  (MCP auto-registration failed — run 'ccguilt --setup-mcp' manually if you want it.)"
    fi
fi

echo ""
echo "Remember: this report was installed using energy. You're welcome, planet."
