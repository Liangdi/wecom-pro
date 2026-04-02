# 构建 npm 包

本指南说明如何为 wecom-pro 构建 npm 包。

## 前置要求

- Rust 工具链（rustc, cargo）
- Node.js 18+
- npm 或 pnpm

## 构建步骤

### 1. 安装 Rust 目标平台

```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc
```

### 2. 运行构建脚本

```bash
./build-npm.sh
```

或者使用 npm：

```bash
npm run build:npm
```

### 3. 测试二进制文件

```bash
# 测试 Linux 二进制
./packages/linux-x64/bin/wecom-pro --version

# 测试 macOS 二进制（如果在 macOS 上）
./packages/darwin-x64/bin/wecom-pro --version

# 测试 Windows 二进制（如果在 Windows 上）
./packages/win32-x64/bin/wecom-pro.exe --version
```

### 4. 发布到 npm

#### 发布平台特定的包

```bash
# 发布 Linux x64
cd packages/linux-x64
npm publish

# 发布 macOS x64
cd ../darwin-x64
npm publish

# 发布 macOS ARM64
cd ../darwin-arm64
npm publish

# 发布 Windows x64
cd ../win32-x64
npm publish
```

#### 发布主包

```bash
cd ../../
npm publish
```

## 跨平台构建

### Linux

在 Linux 上，您可以直接构建 Linux 二进制文件。要构建 macOS 和 Windows 二进制文件，您需要：

#### macOS 交叉编译

```bash
# 安装 macOS 交叉编译工具
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

#### Windows 交叉编译

```bash
# 安装 Windows 交叉编译工具
rustup target add x86_64-pc-windows-msvc
```

### 使用 GitHub Actions（推荐）

最简单的方法是使用 GitHub Actions 自动构建所有平台：

1. 将代码推送到 GitHub
2. GitHub Actions 会自动构建所有平台的二进制文件
3. 从 Artifacts 下载构建好的二进制文件

### 使用 Docker

如果您在 Linux 上需要构建 Windows 二进制文件：

```bash
# 使用 mingw-w64
sudo apt-get install mingw-w64
rustup target add x86_64-pc-windows-msvc
```

## 版本管理

在发布新版本之前，请确保：

1. 更新 `package.json` 中的版本号
2. 更新 `Cargo.toml` 中的版本号
3. 更新所有 `packages/*/package.json` 中的版本号
4. 运行构建脚本
5. 测试所有二进制文件
6. 提交更改并打标签

```bash
# 更新版本
npm version patch  # 或 minor, major

# 构建并测试
./build-npm.sh

# 提交
git add .
git commit -m "chore: release v$(node -p "require('./package.json').version")"
git tag "v$(node -p "require('./package.json').version")"

# 发布
git push && git push --tags
npm publish
```

## 故障排除

### 构建失败

- 确保 Rust 工具链已正确安装
- 检查是否已添加所有目标平台
- 确保有足够的磁盘空间

### 发布失败

- 确保您已登录 npm：`npm login`
- 检查包名是否已被占用
- 确保 package.json 中的所有字段都是正确的
- 检查 `.npmignore` 文件（如果存在）

### 二进制文件不工作

- 使用 `file` 命令检查二进制文件类型
- 确保二进制文件有执行权限
- 在目标平台上测试二进制文件
