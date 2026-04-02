dev:
    cargo run

build:
    cargo build

release:
    cargo build --release


release-patch:
    cargo release patch --no-publish --execute

release-minor:
    cargo release minor --no-publish --execute

release-major:
    cargo release major --no-publish --execute

upgrade:
    cargo +nightly update --breaking -Z unstable-options

# 构建 Linux 和 Windows 平台
npm-build-linux-windows:
    #!/bin/bash
    set -e
    VERSION=$(node -p "require('./package.json').version")
    echo "🚀 Building wecom-pro v$VERSION for Linux/Windows..."

    TARGETS=(
        "x86_64-unknown-linux-gnu:linux-x64"
        "x86_64-pc-windows-msvc:win32-x64"
    )

    for target_pair in "${TARGETS[@]}"; do
        IFS=':' read -r rust_target npm_platform <<< "$target_pair"
        echo "🔨 Building $rust_target ($npm_platform)..."

        rustup target add "$rust_target" 2>/dev/null || true
        cargo build --release --target "$rust_target"

        if [[ "$rust_target" == *"windows"* ]]; then
            BINARY_NAME="wecom-pro.exe"
        else
            BINARY_NAME="wecom-pro"
        fi

        SOURCE="target/$rust_target/release/$BINARY_NAME"
        DEST="packages/$npm_platform/bin"
        mkdir -p "$DEST"
        cp "$SOURCE" "$DEST/"
        echo "✅ Copied to $DEST"
    done
    echo "🎉 Linux/Windows build completed!"

# 构建 Apple 平台（需要在 macOS 上运行）
npm-build-apple:
    #!/bin/bash
    set -e
    VERSION=$(node -p "require('./package.json').version")
    echo "🚀 Building wecom-pro v$VERSION for Apple platforms..."

    TARGETS=(
        "x86_64-apple-darwin:darwin-x64"
        "aarch64-apple-darwin:darwin-arm64"
    )

    for target_pair in "${TARGETS[@]}"; do
        IFS=':' read -r rust_target npm_platform <<< "$target_pair"
        echo "🔨 Building $rust_target ($npm_platform)..."

        rustup target add "$rust_target" 2>/dev/null || true
        cargo build --release --target "$rust_target"

        SOURCE="target/$rust_target/release/wecom-pro"
        DEST="packages/$npm_platform/bin"
        mkdir -p "$DEST"
        cp "$SOURCE" "$DEST/"
        echo "✅ Copied to $DEST"
    done
    echo "🎉 Apple platforms build completed!"

# 发布 Linux 和 Windows 平台包
npm-publish-linux-windows:
    #!/bin/bash
    set -e
    echo "📦 Publishing Linux/Windows packages..."
    cd packages/linux-x64 && npm publish --access public
    cd ../win32-x64 && npm publish --access public
    cd ../..
    echo "✅ Linux/Windows packages published!"

# 发布 Apple 平台包
npm-publish-apple:
    #!/bin/bash
    set -e
    echo "📦 Publishing Apple platform packages..."
    cd packages/darwin-x64 && npm publish --access public
    cd ../darwin-arm64 && npm publish --access public
    cd ../..
    echo "✅ Apple platform packages published!"

# 发布主包
npm-publish:
    npm publish --access public --registry https://registry.npmjs.org/

# Linux/Windows 完整发布流程
npm-publish-linux-windows-all: npm-build-linux-windows npm-publish-linux-windows npm-publish

# Apple 平台完整发布流程
npm-publish-apple-all: npm-build-apple npm-publish-apple npm-publish

cargo-publish:
    cargo publish --registry crates-io 