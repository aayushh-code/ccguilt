#!/bin/bash
# ccguilt installer — curl -sSL http://192.168.100.195/aayush/ccguilt/raw/branch/master/install.sh | bash
set -e

REPO="http://192.168.100.195/aayush/ccguilt"
BINARY_URL="${REPO}/releases/download/v0.2.2/ccguilt-linux-amd64"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="ccguilt"

echo "==================================================================="
echo "  CCGUILT INSTALLER"
echo "  Claude Code Guilt Trip — because the planet wasn't suffering enough"
echo "==================================================================="
echo ""

# Check architecture
ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ]; then
    echo "Error: Only x86_64 (amd64) binaries are available right now."
    echo "Your architecture: $ARCH"
    exit 1
fi

OS=$(uname -s)
if [ "$OS" != "Linux" ]; then
    echo "Error: Only Linux binaries are available right now."
    echo "Your OS: $OS"
    exit 1
fi

echo "Downloading ccguilt v0.2.2..."
TMP=$(mktemp)
if command -v curl &>/dev/null; then
    curl -sSL -o "$TMP" "$BINARY_URL"
elif command -v wget &>/dev/null; then
    wget -q -O "$TMP" "$BINARY_URL"
else
    echo "Error: curl or wget required"
    exit 1
fi

chmod +x "$TMP"

# Try /usr/local/bin first, fall back to ~/.local/bin
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
echo "Setting up shell completions..."
if "${INSTALL_DIR}/${BINARY_NAME}" --setup-completions 2>&1; then
    echo "  Tab completion enabled!"
else
    echo "  Run 'ccguilt --setup-completions' to enable tab completion."
fi

echo ""
echo "Done! Run 'ccguilt daily' to see your environmental destruction."
echo "Run 'ccguilt --help' for all options."
echo ""
echo "Remember: this report was installed using energy. You're welcome, planet."
