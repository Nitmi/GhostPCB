# 应用发布流程

## 版本号位置

版本号需要在以下三个文件中同步更新：

| 文件 | 字段 | 格式 |
|------|------|------|
| `package.json` | `version` | `x.y.z` |
| `src-tauri/tauri.conf.json` | `version` | `x.y.z` |
| `src/App.tsx` | `APP_VERSION` | `vx.y.z` |

## 发布检查清单

### 1. 完成代码迭代

- [ ] 功能开发完成
- [ ] 代码审查通过

### 2. 通过测试

```bash
cd src-tauri
cargo test
```

### 3. 同步完善文档

- [ ] 更新 `README.md`（如有必要）
- [ ] 更新 `.kiro/steering/` 下的相关文档

### 4. 更新版本号

同步修改以下文件中的版本号：

```json
// package.json
{
  "version": "x.y.z"
}
```

```json
// src-tauri/tauri.conf.json
{
  "version": "x.y.z"
}
```

```typescript
// src/App.tsx
const APP_VERSION = "vx.y.z";
```

### 5. 编写 CHANGELOG

在 `CHANGELOG.md` 中添加新版本记录：

```markdown
## [x.y.z] - YYYY-MM-DD

### 新增
- 新功能描述

### 优化
- 优化内容描述

### 修复
- Bug 修复描述
```

### 6. 提交 Git 仓库

```bash
git add .
git commit -m "release: vx.y.z"
```

### 7. 打 Git 标签

```bash
# 创建本地标签
git tag vx.y.z

# 推送到远程
git push origin main
git push origin vx.y.z
```

## 版本号规范

遵循语义化版本 (Semantic Versioning)：

- `MAJOR.MINOR.PATCH`
- MAJOR: 不兼容的 API 变更
- MINOR: 向后兼容的功能新增
- PATCH: 向后兼容的问题修复

## 自动更新机制

应用使用 Tauri Updater 插件实现自动更新：

- 更新检查端点配置在 `tauri.conf.json` 的 `plugins.updater.endpoints`
- GitHub Actions 会自动生成 `latest.json` 更新清单
- 用户启动应用时自动检查更新
