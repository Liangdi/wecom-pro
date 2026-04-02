#!/bin/bash
set -e

echo "🚀 Starting wecom-pro npm package build process..."

# 配置
VERSION=$(node -p "require('./package.json').version")
echo "📦 Building version: $VERSION"

# 定义目标平台
TARGETS=(
    "x86_64-unknown-linux-gnu:linux-x64"
    "x86_64-unknown-linux-musl:linux-x64-musl"
    "x86_64-apple-darwin:darwin-x64"
    "aarch64-apple-darwin:darwin-arm64"
    "x86_64-pc-windows-msvc:win32-x64"
)

# 创建临时目录
TEMP_DIR=$(mktemp -d)
echo "📁 Using temp directory: $TEMP_DIR"

# 构建每个目标
for target_pair in "${TARGETS[@]}"; do
    IFS=':' read -r rust_target npm_platform <<< "$target_pair"
    echo "🔨 Building for $rust_target ($npm_platform)..."

    # 添加 target（如果尚未添加）
    rustup target add "$rust_target" || echo "Target already exists"

    # 构建 release 版本
    if [[ "$rust_target" == *"windows"* ]]; then
        cargo build --release --target "$rust_target"
        BINARY_NAME="wecom-pro.exe"
    else
        cargo build --release --target "$rust_target"
        BINARY_NAME="wecom-pro"
    fi

    # 复制二进制文件
    SOURCE_PATH="target/$rust_target/release/$BINARY_NAME"
    DEST_DIR="packages/$npm_platform/bin"
    mkdir -p "$DEST_DIR"

    if [[ -f "$SOURCE_PATH" ]]; then
        cp "$SOURCE_PATH" "$DEST_DIR/"
        echo "✅ Copied $BINARY_NAME to $DEST_DIR"
    else
        echo "❌ Failed to find binary at $SOURCE_PATH"
        exit 1
    fi
done

echo "🎉 Build completed successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Test the binaries locally"
echo "2. Commit the changes: git add packages/ && git commit -m 'chore: build npm binaries for v$VERSION'"
echo "3. Publish to npm: npm publish"
echo ""
echo "To publish platform-specific packages, run:"
echo "  cd packages/linux-x64 && npm publish"
echo "  cd packages/darwin-x64 && npm publish"
echo "  cd packages/darwin-arm64 && npm publish"
echo "  cd packages/win32-x64 && npm publish"
echo "  cd .. && npm publish"
