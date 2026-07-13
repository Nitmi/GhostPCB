# 前端开发指南

## UI 设计要点

### 主界面布局

```
┌─────────────────────────────────────────┐
│  GhostPCB - Gerber 指纹混淆工具          │
├─────────────────────────────────────────┤
│  📁 选择 Gerber 文件                     │
│  ┌─────────────────────────────────────┐│
│  │ [拖拽或点击选择 .zip 文件]           ││
│  └─────────────────────────────────────┘│
│                                         │
│  ⚙️ 混淆选项                             │
│  ☑ 时间戳修改                           │
│  ☑ 丝印层扰动                           │
│  ☑ 几何结构扰动                         │
│  ☑ 文件结构混淆                         │
│  ☑ 物理参数微调                         │
│                                         │
│  📊 生成设置                             │
│  生成数量: [___3___] 个                  │
│  输出目录: [默认] [选择...]              │
│                                         │
│  [        开始处理        ]              │
│                                         │
│  📋 处理日志                             │
│  ┌─────────────────────────────────────┐│
│  │ ...                                 ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

### 组件结构

```
src/
├── App.tsx
├── App.css
├── components/
│   ├── FileDropZone.tsx      # 文件拖拽区
│   ├── OptionsPanel.tsx      # 混淆选项面板
│   ├── GenerateSettings.tsx  # 生成设置
│   ├── ProcessButton.tsx     # 处理按钮
│   └── LogPanel.tsx          # 日志面板
├── hooks/
│   └── useGerberProcess.ts   # 处理逻辑 hook
└── types/
    └── index.ts              # 类型定义
```

## Tauri API 调用

### 调用 Rust 命令

```typescript
import { invoke } from '@tauri-apps/api/core';

interface ProcessRequest {
  input_path: string;
  output_dir: string | null;
  count: number;
  options: {
    timestamp: boolean;
    silkscreen: boolean;
    geometry: boolean;
    structure: boolean;
    physical: boolean;
  };
}

interface ProcessResult {
  success: boolean;
  output_files: string[];
  message: string;
}

async function processGerber(request: ProcessRequest): Promise<ProcessResult> {
  return await invoke('process_gerber', { request });
}
```

### 文件选择对话框

```typescript
import { open } from '@tauri-apps/plugin-dialog';

async function selectGerberFile(): Promise<string | null> {
  const selected = await open({
    filters: [{
      name: 'Gerber ZIP',
      extensions: ['zip']
    }],
    multiple: false
  });
  return selected as string | null;
}

async function selectOutputDir(): Promise<string | null> {
  const selected = await open({
    directory: true
  });
  return selected as string | null;
}
```

## 状态管理

使用 React useState 管理简单状态即可：

```typescript
const [inputFile, setInputFile] = useState<string | null>(null);
const [options, setOptions] = useState<ObfuscateOptions>(defaultOptions);
const [count, setCount] = useState(3);
const [outputDir, setOutputDir] = useState<string | null>(null);
const [processing, setProcessing] = useState(false);
const [logs, setLogs] = useState<string[]>([]);
```

## 需要安装的前端依赖

```bash
pnpm add @tauri-apps/plugin-dialog
```

同时需要在 Rust 端添加对应插件：

```toml
# src-tauri/Cargo.toml
tauri-plugin-dialog = "2"
```

并在 `tauri.conf.json` 中配置权限。
