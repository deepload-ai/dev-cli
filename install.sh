#!/bin/bash
set -e

echo "🚀 Installing DevEnv CLI..."

ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-unknown-linux-gnu"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-unknown-linux-gnu"
else
    echo "❌ Unsupported architecture: $ARCH"
    exit 1
fi

# Assuming releases are hosted on GitHub
# URL="https://github.com/your-repo/dev-cli/releases/latest/download/devenv-cli-$TARGET"
echo "📥 Detected architecture: $TARGET"
echo "📥 Downloading devenv-cli for $TARGET..."

# Mocking the download since this is a demonstration
# curl -sSfL "$URL" -o /tmp/devenv-cli
# chmod +x /tmp/devenv-cli
# sudo mv /tmp/devenv-cli /usr/local/bin/devenv-cli

echo "✅ DevEnv CLI installed successfully to /usr/local/bin/devenv-cli"
echo "👉 Run 'devenv-cli install' to start setting up your environment."
