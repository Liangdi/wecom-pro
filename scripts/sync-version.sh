#!/bin/bash
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Error: Version argument is required"
    echo "Usage: $0 <version>"
    exit 1
fi

echo "Syncing version to $VERSION in all package.json files..."

# 更新根目录的 package.json
echo "Updating root package.json..."
npm version "$VERSION" --no-git-tag-version --workspaces-update false

# 更新平台特定的包
echo "Updating linux-x64 package.json..."
cd packages/linux-x64 && npm version "$VERSION" --no-git-tag-version && cd ../..

echo "Updating darwin-x64 package.json..."
cd packages/darwin-x64 && npm version "$VERSION" --no-git-tag-version && cd ../..

echo "Updating darwin-arm64 package.json..."
cd packages/darwin-arm64 && npm version "$VERSION" --no-git-tag-version && cd ../..

echo "Updating win32-x64 package.json..."
cd packages/win32-x64 && npm version "$VERSION" --no-git-tag-version && cd ../..

echo "All package.json files updated to version $VERSION"
