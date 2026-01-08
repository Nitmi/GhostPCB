# GhostPCB 项目概述

## 项目定位

GhostPCB 是一个基于 Tauri 的桌面应用，用于对 PCB 的 Gerber 文件进行指纹混淆处理，防止 PCB 生产商通过文件特征识别出相同的设计文件。

## 技术栈

- **前端**: React 19 + TypeScript + Vite
- **后端**: Rust (Tauri 2.x)
- **核心处理**: Rust 实现所有 Gerber 文件处理逻辑

## 核心功能

1. **ZIP 文件处理**: 读取 Gerber ZIP 包，处理后重新打包
2. **多种混淆策略**: 时间戳修改、丝印扰动、几何扰动、结构混淆、物理参数微调
3. **批量生成**: 支持一次生成多个不同指纹的 Gerber 文件
4. **可配置选项**: 用户可选择启用/禁用各种混淆策略

## 输出规范

- 默认输出目录: `原Gerber同级目录/GhostPCB_YYYYMMDD_原文件名/`
- 输出文件命名: `原文件名_序号.zip`
- 支持自定义输出路径

## 项目结构

```
GhostPCB/
├── src/                    # React 前端
│   ├── App.tsx            # 主界面
│   ├── components/        # UI 组件
│   └── ...
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── lib.rs         # Tauri 入口
│   │   ├── commands/      # Tauri 命令
│   │   └── gerber/        # Gerber 处理核心模块
│   └── Cargo.toml
└── docs/                  # 文档
```
