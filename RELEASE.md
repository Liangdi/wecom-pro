# 发布流程

## 版本发布

本项目使用 `cargo release` 工具进行版本发布，它会自动同步更新所有 `package.json` 文件的版本号。

### 安装 cargo-release

```bash
cargo install cargo-release
```

### 发布流程

#### 1. 查看当前版本

```bash
# 查看 Cargo.toml 中的版本
grep version Cargo.toml

# 查看所有 package.json 中的版本
grep -h '"version"' package.json packages/*/package.json
```

#### 2. 测试发布（不执行实际操作）

```bash
# 小版本更新（例如 0.1.5 -> 0.2.0）
cargo release --level minor --dry-run

# 补丁更新（例如 0.1.5 -> 0.1.6）
cargo release --level patch --dry-run

# 主版本更新（例如 0.1.5 -> 1.0.0）
cargo release --level major --dry-run
```

#### 3. 执行发布

```bash
# 补丁更新（Bug 修复）
cargo release --level patch

# 小版本更新（新功能）
cargo release --level minor

# 主版本更新（破坏性变更）
cargo release --level major

# 指定特定版本
cargo release --version 0.2.0
```

### 发布流程说明

当你执行 `cargo release` 时，它会：

1. **更新版本号**
   - 更新 `Cargo.toml` 中的版本
   - 自动触发 `pre-release-hook`，运行 `scripts/sync-version.sh`
   - 同步更新所有 `package.json` 文件（根目录 + 4 个平台包）

2. **更新文档**
   - 在 `README.md` 中更新版本号引用

3. **创建 Git 提交**
   - 提交所有版本变更
   - 创建 Git tag（格式：`v{version}`）

4. **推送标签**
   - 推送提交和标签到远程仓库

### 手动同步版本号

如果需要手动同步版本号（不使用 `cargo release`）：

```bash
# 使用同步脚本
./scripts/sync-version.sh 0.1.6
```

### 发布到 npm

版本发布后，需要手动发布到 npm：

```bash
# 使用 npm 脚本发布所有平台包
npm run publish:all

# 或手动逐个发布
cd packages/linux-x64 && npm publish && cd ../..
cd packages/darwin-x64 && npm publish && cd ../..
cd packages/darwin-arm64 && npm publish && cd ../..
cd packages/win32-x64 && npm publish && cd ../..
npm publish
```

### 注意事项

1. **版本号同步**：所有 `package.json` 文件必须与 `Cargo.toml` 版本号保持一致
2. **发布前测试**：建议先使用 `--dry-run` 参数测试发布流程
3. **Git 状态**：发布前确保工作区干净（无未提交的更改）
4. **npm 认证**：发布到 npm 前，确保已登录 (`npm login`)

### 回滚版本

如果发布出错，可以回滚版本：

```bash
# 删除本地标签
git tag -d v0.1.6

# 删除远程标签
git push origin :refs/tags/v0.1.6

# 恢复到之前的版本
./scripts/sync-version.sh 0.1.5
git add .
git commit -m "chore: revert version to 0.1.5"
git push
```
