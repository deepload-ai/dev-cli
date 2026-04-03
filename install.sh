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

# We fetch the latest release from GitHub
REPO="deepload-ai/dev-cli"
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    # Fallback if no release exists yet
    echo "⚠️  No GitHub release found for $REPO yet. The CLI might not be published."
    echo "⚠️  Please clone the repository and build from source using 'cargo build --release'."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/devenv-cli-$TARGET"
echo "📥 Downloading devenv-cli $LATEST_RELEASE for $TARGET..."

curl -sSfL "$URL" -o /tmp/devenv-cli
chmod +x /tmp/devenv-cli
sudo mv /tmp/devenv-cli /usr/local/bin/devenv-cli

echo "✅ DevEnv CLI installed successfully to /usr/local/bin/devenv-cli"
echo "👉 Run 'devenv-cli install' to start setting up your environment."
