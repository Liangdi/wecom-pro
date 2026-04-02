# npm 包发布指南

## 概述

wecom-pro 现在支持以 npm 包形式发布，这使得安装更加方便，同时保持了 Rust 的高性能特性。

## 发布流程

### 1. 准备工作

确保您已完成以下步骤：

- ✅ 更新了 `package.json` 中的包名为 `@liangdi/wecom-pro`
- ✅ 更新了所有平台特定的 `package.json` 文件
- ✅ 更新了 `bin/wecom.js` 中的包名引用
- ✅ 创建了构建脚本 `build-npm.sh`

### 2. 登录 npm

```bash
npm login
# 输入您的 npm 凭证
```

### 3. 检查包名是否可用

```bash
npm search @liangdi/wecom-pro
# 如果没有结果，说明包名可用
```

### 4. 构建所有平台的二进制文件

```bash
# 方式一：使用构建脚本（推荐）
./build-npm.sh

# 方式二：使用 npm 脚本
npm run build:npm

# 方式三：单独构建某个平台
npm run build:linux
npm run build:darwin-x64
npm run build:darwin-arm64
npm run build:windows
```

**注意**：跨平台构建需要相应的工具链：
- **Linux**：可以直接构建 Linux 二进制文件
- **macOS**：需要 macOS 系统或使用交叉编译
- **Windows**：需要 Windows 系统或使用 mingw-w64

### 5. 验证构建结果

```bash
# 检查二进制文件是否存在
ls -la packages/*/bin/

# 测试二进制文件（在当前平台上）
./packages/linux-x64/bin/wecom-pro --version
```

### 6. 发布平台特定的包

```bash
# Linux x64
cd packages/linux-x64
npm publish

# macOS x64
cd ../darwin-x64
npm publish

# macOS ARM64
cd ../darwin-arm64
npm publish

# Windows x64
cd ../win32-x64
npm publish
```

### 7. 发布主包

```bash
cd ../..
npm publish
```

或者使用一键发布脚本：

```bash
npm run publish:all
```

## 验证发布

### 本地测试

```bash
# 创建临时目录测试安装
cd /tmp
mkdir test-wecom-pro
cd test-wecom-pro

# 初始化 npm 项目
npm init -y

# 安装包
npm install @liangdi/wecom-pro

# 测试运行
npx wecom-pro --version
./node_modules/.bin/wecom-pro --help
```

### 公开测试

发布后，用户可以通过以下方式安装：

```bash
# 全局安装
npm install -g @liangdi/wecom-pro

# 使用 npx（无需安装）
npx @liangdi/wecom-pro --help

# 在项目中使用
npm install @liangdi/wecom-pro
```

## 版本管理

### 更新版本号

```bash
# 更新所有相关文件中的版本号
npm version patch  # 0.1.3 -> 0.1.4
npm version minor  # 0.1.3 -> 0.2.0
npm version major  # 0.1.3 -> 1.0.0
```

这会自动更新：
- `package.json`
- `Cargo.toml`
- `packages/*/package.json`

### 发布新版本

```bash
# 1. 更新版本
npm version patch

# 2. 构建二进制文件
./build-npm.sh

# 3. 提交并打标签
git add .
git commit -m "chore: release v$(node -p "require('./package.json').version")"
git tag "v$(node -p "require('./package.json').version")"

# 4. 推送到 GitHub
git push && git push --tags

# 5. 发布到 npm
npm run publish:all
```

## CI/CD 自动化

推荐使用 GitHub Actions 自动化构建和发布流程：

```yaml
name: Build and Publish

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            platform: linux-x64
          - os: macos-latest
            target: x86_64-apple-darwin
            platform: darwin-x64
          - os: macos-latest
            target: aarch64-apple-darwin
            platform: darwin-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            platform: win32-x64

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          target: ${{ matrix.target }}
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}
          path: target/${{ matrix.target }}/release/wecom-pro*
```

## 故障排除

### 常见问题

1. **权限错误**
   ```bash
   npm ERR! 403 403 Forbidden
   ```
   - 确保您是包的维护者
   - 检查包名是否已被占用

2. **构建失败**
   ```bash
   error: linker not found
   ```
   - 安装必要的构建工具
   - Linux: `sudo apt-get install build-essential`
   - macOS: `xcode-select --install`

3. **二进制文件太大**
   - 检查 `Cargo.toml` 中的 release 配置
   - 确保启用了 `strip = true`

4. **平台不兼容**
   - 检查目标平台是否正确
   - 使用正确的目标 triple（如 `x86_64-unknown-linux-gnu`）

## 相关文件

- `build-npm.sh` - 构建脚本
- `bin/wecom.js` - Node.js 包装器
- `packages/*/package.json` - 平台特定的包配置
- `.npmignore` - npm 打包忽略文件

## 支持

如有问题，请提交 Issue：https://github.com/Liangdi/wecom-pro/issues
