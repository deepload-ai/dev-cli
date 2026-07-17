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
API_URL="https://api.github.com/repos/$REPO/releases/latest"
if ! RELEASE_JSON=$(curl -fsSL \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "$API_URL"); then
    echo "❌ Failed to query the latest GitHub release for $REPO."
    echo "⚠️  This is usually caused by network issues, GitHub API rate limiting, or temporary GitHub errors."
    echo "⚠️  Please retry later, or build from source using 'cargo build --release'."
    exit 1
fi

LATEST_RELEASE=$(printf '%s\n' "$RELEASE_JSON" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1)

if [ -z "$LATEST_RELEASE" ]; then
    echo "❌ GitHub API did not return a valid latest release tag for $REPO."
    echo "⚠️  Please retry later, or build from source using 'cargo build --release'."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/devenv-cli-$TARGET"
echo "📥 Downloading devenv-cli $LATEST_RELEASE for $TARGET..."

if ! curl -sSfL "$URL" -o /tmp/devenv-cli; then
    echo "❌ Found release $LATEST_RELEASE, but no downloadable asset was available for $TARGET."
    echo "⚠️  Please check the GitHub release assets for your architecture, or build from source using 'cargo build --release'."
    exit 1
fi
chmod +x /tmp/devenv-cli
sudo mv /tmp/devenv-cli /usr/local/bin/devenv-cli

echo "✅ DevEnv CLI installed successfully to /usr/local/bin/devenv-cli"
echo "👉 Run 'devenv-cli install' to start setting up your environment."
