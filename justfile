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

# 构建 npm 包（所有平台）
npm-build:
    #!/bin/bash
    set -e
    VERSION=$(node -p "require('./package.json').version")
    echo "🚀 Building wecom-pro v$VERSION for npm..."

    TARGETS=(
        "x86_64-unknown-linux-gnu:linux-x64"
        "x86_64-apple-darwin:darwin-x64"
        "aarch64-apple-darwin:darwin-arm64"
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
    echo "🎉 Build completed!"

# 发布平台特定包
npm-publish-platforms:
    #!/bin/bash
    set -e
    echo "📦 Publishing platform-specific packages..."
    cd packages/linux-x64 && npm publish --access public
    cd ../darwin-x64 && npm publish --access public
    cd ../darwin-arm64 && npm publish --access public
    cd ../win32-x64 && npm publish --access public
    cd ../..
    echo "✅ Platform packages published!"

# 发布主包
npm-publish:
    npm publish --access public --registry https://registry.npmjs.org/

# 完整发布流程：构建 + 发布所有包
npm-publish-all: npm-build npm-publish-platforms npm-publish

cargo-publish:
    cargo publish --registry crates-io 